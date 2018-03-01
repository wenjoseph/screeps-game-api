use std::{fs, process, ffi::OsStr, path::Path};

use {failure, regex};

// __initialize defined by stdweb.
// it's signature is 'function __initialize( __wasm_module, __load_asynchronously ) {'
pub static SCREEPS_JS_INITIALIZE_CALL: &str = r#"
__initialize(new WebAssembly.Module(require('compiled')), false);
"#;

pub fn check(root: &Path) -> Result<(), failure::Error> {
    debug!("running check");

    debug!("running 'cargo check --target=wasm32-unknown-unknown'");
    let cargo_success = process::Command::new("cargo")
        .args(&["check", "--target=wasm32-unknown-unknown"])
        .current_dir(root)
        .spawn()?
        .wait()?;
    if !cargo_success.success() {
        bail!(
            "'cargo check' exited with a non-zero exit code: {}",
            cargo_success
        );
    }

    debug!("finished 'cargo check'");
    Ok(())
}

pub fn build(root: &Path) -> Result<(), failure::Error> {
    debug!("building");

    debug!("running 'cargo web build --target=wasm32-unknown-unknown --release'");
    let cargo_success = process::Command::new("cargo")
        .args(&[
            "web",
            "build",
            "--target=wasm32-unknown-unknown",
            "--release",
        ])
        .current_dir(root)
        .spawn()?
        .wait()?;
    if !cargo_success.success() {
        bail!(
            "'cargo web' exited with a non-zero exit code: {}",
            cargo_success
        );
    }

    debug!("finished 'cargo web'");

    let target_dir = root.join("target/wasm32-unknown-unknown/release/");
    // TODO: actually use 'cargo metadata' to get exact filename that will be
    // built, rather than using this hack.
    let mut wasm_file = None;
    let mut generated_js = None;
    for r in fs::read_dir(&target_dir)? {
        let entry = r?;
        let file_name = entry.file_name();
        let file_name = Path::new(&file_name);
        match file_name.extension().and_then(OsStr::to_str) {
            Some("wasm") => {
                ensure!(
                    wasm_file.is_none(),
                    "error: multiple wasm files found in {}",
                    target_dir.display()
                );
                wasm_file = Some(entry.path());
            }
            Some("js") => {
                ensure!(
                    generated_js.is_none(),
                    "error: multiple js files found in {}",
                    target_dir.display()
                );
                generated_js = Some(entry.path());
            }
            _ => {}
        }
    }
    let wasm_file = wasm_file
        .ok_or_else(|| format_err!("error: no wasm files found in {}", target_dir.display()))?;
    let generated_js = generated_js
        .ok_or_else(|| format_err!("error: no js files found in {}", target_dir.display()))?;

    let out_dir = root.join("target");

    debug!("copying wasm file");

    fs::create_dir_all(&out_dir)?;

    fs::copy(wasm_file, out_dir.join("compiled.wasm"))?;

    debug!("processing js file");

    fs::write(
        out_dir.join("main.js"),
        process_js(&generated_js, &fs::read_string(&generated_js)?)?,
    )?;

    Ok(())
}

fn process_js(file_name: &Path, input: &str) -> Result<String, failure::Error> {
    // first, strip out bootstrap code which relates to the browser. We don't want
    // to run this, we just want to call `__initialize` ourself.
    //
    // TODO: this is currently quite brittle and tied to the
    // version of "cargo web"...
    let whitespace_regex = regex::Regex::new("\\s+").expect("expected pre-set regex to succeed");
    let make_into_slightly_less_brittle_regex = |input: &str| {
        whitespace_regex
            .replace_all(&regex::escape(input), "\\s*")
            .replace("XXX", "[A-Za-z0-9_]*")
    };
    let expected_prefix = r#""use strict";

if( typeof Rust === "undefined" ) {
    var Rust = {};
}

(function( root, factory ) {
    if( typeof define === "function" && define.amd ) {
        define( [], factory );
    } else if( typeof module === "object" && module.exports ) {
        module.exports = factory();
    } else {
        Rust.XXX = factory();
    }
}( this, function() {
    "#;

    let expected_prefix = regex::Regex::new(&format!(
        "^{}",
        make_into_slightly_less_brittle_regex(expected_prefix)
    ))?;

    debug!("expected prefix:\n```{}```", expected_prefix);

    let expected_suffix = r#"


    if( typeof window === "undefined" ) {
        const fs = require( "fs" );
        const path = require( "path" );
        const wasm_path = path.join( __dirname, "XXX.wasm" );
        const buffer = fs.readFileSync( wasm_path );
        const mod = new WebAssembly.Module( buffer );

        return __initialize( mod, false );
    } else {
        return fetch( "XXX.wasm" )
            .then( response => response.arrayBuffer() )
            .then( bytes => WebAssembly.compile( bytes ) )
            .then( mod => __initialize( mod, true ) );
    }
}));
"#;

    let expected_suffix = regex::Regex::new(&format!(
        "{}$",
        make_into_slightly_less_brittle_regex(expected_suffix)
    ))?;

    debug!("expected suffix:\n```{}```", expected_suffix);

    let (prefix_match, suffix_match) = expected_prefix
        .find(input)
        .and_then(|a| expected_suffix.find(input).map(|b| (a, b)))
        .ok_or_else(|| {
            format_err!(
                "'cargo web' generated unexpected JS prefix! This means it's updated without \
                 'cargo screeps' also having updates. Please report this issue to \
                 https://github.com/daboross/screeps-in-rust-via-wasm/issues and include \
                 the first ~30 lines of {}",
                file_name.display(),
            )
        })?;

    ensure!(
        input.contains("__initialize"),
        "'cargo web' generated unexpected JS output! It does not \
         include a '__initialize' function. Please report this issue to \
         https://github.com/daboross/screeps-in-rust-via-wasm/issues."
    );

    let initialize_function = &input[prefix_match.end()..suffix_match.start()];

    Ok(format!(
        "{}\n{}",
        initialize_function, SCREEPS_JS_INITIALIZE_CALL
    ))
}
