//! `*Type` constants.
use std::{borrow::Cow, str::FromStr};

use num_derive::FromPrimitive;
use parse_display::{Display, FromStr};
use serde::{
    de::{Deserializer, Error as _, Unexpected},
    Deserialize, Serialize, Serializer,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Translates `STRUCTURE_*` constants.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__structure_type_num_to_str` and
/// `__structure_type_str_to_num` JavaScript functions,
/// [`FromStr`][std::str::FromStr] or [`StructureType::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(
    Copy, Clone, Debug, Display, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr,
)]
#[repr(u8)]
#[display(style = "camelCase")]
pub enum StructureType {
    Spawn = 0,
    Extension = 1,
    Road = 2,
    Wall = 3,
    Rampart = 4,
    KeeperLair = 5,
    Portal = 6,
    Controller = 7,
    Link = 8,
    Storage = 9,
    Tower = 10,
    Observer = 11,
    PowerBank = 12,
    PowerSpawn = 13,
    Extractor = 14,
    Lab = 15,
    Terminal = 16,
    Container = 17,
    Nuker = 18,
    Factory = 19,
    InvaderCore = 20,
}

impl StructureType {
    /// Translates the `CONSTRUCTION_COST` constant.
    #[inline]
    pub fn construction_cost(self) -> Option<u32> {
        use self::StructureType::*;

        let cost = match self {
            Spawn => 15_000,
            Extension => 3_000,
            Road => 300,
            Wall => 1,
            Rampart => 1,
            Link => 5_000,
            Storage => 30_000,
            Tower => 5_000,
            Observer => 8_000,
            PowerSpawn => 100_000,
            Extractor => 5_000,
            Lab => 50_000,
            Terminal => 100_000,
            Container => 5_000,
            Nuker => 100_000,
            Factory => 100_000,
            KeeperLair | PowerBank | Portal | Controller | InvaderCore => return None,
        };
        Some(cost)
    }

    /// Translates the `CONTROLLER_STRUCTURES` constant
    #[inline]
    pub fn controller_structures(self, current_rcl: u32) -> u32 {
        use self::StructureType::*;

        match self {
            Spawn => match current_rcl {
                0 => 0,
                1..=6 => 1,
                7 => 2,
                _ => 3,
            },
            Extension => match current_rcl {
                0 | 1 => 0,
                2 => 5,
                3 => 10,
                4 => 20,
                5 => 30,
                6 => 40,
                7 => 50,
                _ => 60,
            },
            Road => 2500,
            Wall => match current_rcl {
                0 | 1 => 0,
                _ => 2500,
            },
            Rampart => match current_rcl {
                0 | 1 => 0,
                _ => 2500,
            },
            Link => match current_rcl {
                0..=4 => 0,
                5 => 2,
                6 => 3,
                7 => 4,
                _ => 6,
            },
            Storage => match current_rcl {
                0..=3 => 0,
                _ => 1,
            },
            Tower => match current_rcl {
                0 | 1 | 2 => 0,
                3 | 4 => 1,
                5 | 6 => 2,
                7 => 3,
                _ => 6,
            },
            Observer => match current_rcl {
                0..=7 => 0,
                _ => 1,
            },
            PowerSpawn => match current_rcl {
                0..=7 => 0,
                _ => 1,
            },
            Extractor => match current_rcl {
                0..=5 => 0,
                _ => 1,
            },
            Lab => match current_rcl {
                0..=5 => 0,
                6 => 3,
                7 => 6,
                _ => 10,
            },
            Terminal => match current_rcl {
                0..=5 => 0,
                _ => 1,
            },
            Container => 5,
            Nuker => match current_rcl {
                0..=7 => 0,
                _ => 1,
            },
            Factory => match current_rcl {
                0..=6 => 0,
                _ => 1,
            },
            KeeperLair | PowerBank | Portal | Controller | InvaderCore => 0,
        }
    }

    /// Translates the `*_HITS` constants, initial hits for structures
    #[inline]
    pub fn initial_hits(self) -> Option<u32> {
        use self::StructureType::*;
        use super::numbers::*;

        let hits = match self {
            Spawn => SPAWN_HITS,
            Extension => EXTENSION_HITS,
            Road => ROAD_HITS,
            Wall => WALL_HITS,
            Rampart => RAMPART_HITS,
            Link => LINK_HITS,
            Storage => STORAGE_HITS,
            Tower => TOWER_HITS,
            Observer => OBSERVER_HITS,
            PowerBank => POWER_BANK_HITS,
            PowerSpawn => POWER_SPAWN_HITS,
            Extractor => EXTENSION_HITS,
            Lab => LAB_HITS,
            Terminal => TOWER_HITS,
            Container => CONTAINER_HITS,
            Nuker => NUKER_HITS,
            Factory => FACTORY_HITS,
            InvaderCore => INVADER_CORE_HITS,
            KeeperLair | Portal | Controller => return None,
        };
        Some(hits)
    }

    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(Unexpected::Str(&s), &"a known STRUCTURE_* constant string")
        })
    }
}

js_deserializable!(StructureType);

/// Translates `SUBSCRIPTION_TOKEN` and `INTERSHARD_RESOURCES` constants.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__resource_type_num_to_str`
/// and `__resource_type_str_to_num` JavaScript functions,
/// [`FromStr`][std::str::FromStr] or
/// [`IntershardResourceType::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(
    Copy, Clone, Debug, Display, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr,
)]
#[repr(u16)]
pub enum IntershardResourceType {
    /// `"token"`
    #[display("token")]
    SubscriptionToken = 1001,
}

impl IntershardResourceType {
    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(
                Unexpected::Str(&s),
                &"a known constant string in INTERSHARD_RESOURCES",
            )
        })
    }
}

js_deserializable!(IntershardResourceType);

/// Resource type constant for all possible types of resources.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__resource_type_num_to_str`
/// and `__resource_type_str_to_num` JavaScript functions,
/// [`FromStr`][std::str::FromStr] or [`ResourceType::deserialize_from_str`].
///
/// See the [module-level documentation][crate::constants] for more details.
#[derive(
    Copy, Clone, Debug, Display, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr,
)]
#[repr(u16)]
pub enum ResourceType {
    /// `"energy"`
    #[display("energy")]
    Energy = 1,
    /// `"power"`
    #[display("power")]
    Power = 2,
    /// `"H"`
    #[display("H")]
    Hydrogen = 3,
    /// `"O"`
    #[display("O")]
    Oxygen = 4,
    /// `"U"`
    #[display("U")]
    Utrium = 5,
    /// `"L"`
    #[display("L")]
    Lemergium = 6,
    /// `"K"`
    #[display("K")]
    Keanium = 7,
    /// `"Z"`
    #[display("Z")]
    Zynthium = 8,
    /// `"X"`
    #[display("X")]
    Catalyst = 9,
    /// `"G"`
    #[display("G")]
    Ghodium = 10,
    // constants.js has these base commodities ordered here, but they're assigned
    // higher integer representations and implemented below to avoid renumbering:
    // RESOURCE_SILICON, RESOURCE_METAL, RESOURCE_BIOMASS, RESOURCE_MIST
    /// `"OH"`
    #[display("OH")]
    Hydroxide = 11,
    /// `"ZK"`
    #[display("ZK")]
    ZynthiumKeanite = 12,
    /// `"UL"`
    #[display("UL")]
    UtriumLemergite = 13,
    /// `"UH"`
    #[display("UH")]
    UtriumHydride = 14,
    /// `"UO"`
    #[display("UO")]
    UtriumOxide = 15,
    /// `"KH"`
    #[display("KH")]
    KeaniumHydride = 16,
    /// `"KO"`
    #[display("KO")]
    KeaniumOxide = 17,
    /// `"LH"`
    #[display("LH")]
    LemergiumHydride = 18,
    /// `"LO"`
    #[display("LO")]
    LemergiumOxide = 19,
    /// `"ZH"`
    #[display("ZH")]
    ZynthiumHydride = 20,
    /// `"ZO"`
    #[display("ZO")]
    ZynthiumOxide = 21,
    /// `"GH"`
    #[display("GH")]
    GhodiumHydride = 22,
    /// `"GO"`
    #[display("GO")]
    GhodiumOxide = 23,
    /// `"UH2O"`
    #[display("UH2O")]
    UtriumAcid = 24,
    /// `"UHO2"`
    #[display("UHO2")]
    UtriumAlkalide = 25,
    /// `"KH2O"`
    #[display("KH2O")]
    KeaniumAcid = 26,
    /// `"KHO2"`
    #[display("KHO2")]
    KeaniumAlkalide = 27,
    /// `"LH2O"`
    #[display("LH2O")]
    LemergiumAcid = 28,
    /// `"LHO2"`
    #[display("LHO2")]
    LemergiumAlkalide = 29,
    /// `"ZH2O"`
    #[display("ZH2O")]
    ZynthiumAcid = 30,
    /// `"ZHO2"`
    #[display("ZHO2")]
    ZynthiumAlkalide = 31,
    /// `"GH2O"`
    #[display("GH2O")]
    GhodiumAcid = 32,
    /// `"GHO2"`
    #[display("GHO2")]
    GhodiumAlkalide = 33,
    /// `"XUH2O"`
    #[display("XUH2O")]
    CatalyzedUtriumAcid = 34,
    /// `"XUHO2"`
    #[display("XUHO2")]
    CatalyzedUtriumAlkalide = 35,
    /// `"XKH2O"`
    #[display("XKH2O")]
    CatalyzedKeaniumAcid = 36,
    /// `"XKHO2"`
    #[display("XKHO2")]
    CatalyzedKeaniumAlkalide = 37,
    /// `"XLH2O"`
    #[display("XLH2O")]
    CatalyzedLemergiumAcid = 38,
    /// `"XLHO2"`
    #[display("XLHO2")]
    CatalyzedLemergiumAlkalide = 39,
    /// `"XZH2O"`
    #[display("XZH2O")]
    CatalyzedZynthiumAcid = 40,
    /// `"XZHO2"`
    #[display("XZHO2")]
    CatalyzedZynthiumAlkalide = 41,
    /// `"XGH2O"`
    #[display("XGH2O")]
    CatalyzedGhodiumAcid = 42,
    /// `"XGHO2"`
    #[display("XGHO2")]
    CatalyzedGhodiumAlkalide = 43,
    /// `"ops"`
    #[display("ops")]
    Ops = 44,
    // these 4 base commodities are ordered earlier in constants.js
    /// `"silicon"`
    #[display("silicon")]
    Silicon = 45,
    /// `"metal"`
    #[display("metal")]
    Metal = 46,
    /// `"biomass"`
    #[display("biomass")]
    Biomass = 47,
    /// `"mist"`
    #[display("mist")]
    Mist = 48,
    /// `"utrium_bar"`
    #[display("utrium_bar")]
    UtriumBar = 49,
    /// `"lemergium_bar"`
    #[display("lemergium_bar")]
    LemergiumBar = 50,
    /// `"zynthium_bar"`
    #[display("zynthium_bar")]
    ZynthiumBar = 51,
    /// `"keanium_bar"`
    #[display("keanium_bar")]
    KeaniumBar = 52,
    /// `"ghodium_melt"`
    #[display("ghodium_melt")]
    GhodiumMelt = 53,
    /// `"oxidant"`
    #[display("oxidant")]
    Oxidant = 54,
    /// `"reductant"`
    #[display("reductant")]
    Reductant = 55,
    /// `"purifier"`
    #[display("purifier")]
    Purifier = 56,
    /// `"battery"`
    #[display("battery")]
    Battery = 57,
    /// `"composite"`
    #[display("composite")]
    Composite = 58,
    /// `"crystal"`
    #[display("crystal")]
    Crystal = 59,
    /// `"liquid"`
    #[display("liquid")]
    Liquid = 60,
    /// `"wire"`
    #[display("wire")]
    Wire = 61,
    /// `"switch"`
    #[display("switch")]
    Switch = 62,
    /// `"transistor"`
    #[display("transistor")]
    Transistor = 63,
    /// `"microchip"`
    #[display("microchip")]
    Microchip = 64,
    /// `"circuit"`
    #[display("circuit")]
    Circuit = 65,
    /// `"device"`
    #[display("device")]
    Device = 66,
    /// `"cell"`
    #[display("cell")]
    Cell = 67,
    /// `"phlegm"`
    #[display("phlegm")]
    Phlegm = 68,
    /// `"tissue"`
    #[display("tissue")]
    Tissue = 69,
    /// `"muscle"`
    #[display("muscle")]
    Muscle = 70,
    /// `"organoid"`
    #[display("organoid")]
    Organoid = 71,
    /// `"organism"`
    #[display("organism")]
    Organism = 72,
    /// `"alloy"`
    #[display("alloy")]
    Alloy = 73,
    /// `"tube"`
    #[display("tube")]
    Tube = 74,
    /// `"fixtures"`
    #[display("fixtures")]
    Fixtures = 75,
    /// `"frame"`
    #[display("frame")]
    Frame = 76,
    /// `"hydraulics"`
    #[display("hydraulics")]
    Hydraulics = 77,
    /// `"machine"`
    #[display("machine")]
    Machine = 78,
    /// `"condensate"`
    #[display("condensate")]
    Condensate = 79,
    /// `"concentrate"`
    #[display("concentrate")]
    Concentrate = 80,
    /// `"extract"`
    #[display("extract")]
    Extract = 81,
    /// `"spirit"`
    #[display("spirit")]
    Spirit = 82,
    /// `"emanation"`
    #[display("emanation")]
    Emanation = 83,
    /// `"essence"`
    #[display("essence")]
    Essence = 84,
}

#[derive(Copy, Clone, Debug)]
pub enum Boost {
    Harvest(f64),
    BuildAndRepair(f64),
    Dismantle(f64),
    UpgradeController(f64),
    Attack(f64),
    RangedAttack(f64),
    Heal(f64),
    Carry(f64),
    Move(f64),
    Tough(f64),
}

impl ResourceType {
    /// Translates the `BOOSTS` constant.
    #[inline]
    pub fn boost(self) -> Option<Boost> {
        use ResourceType::*;
        let boost = match self {
            // these comments copied directly from JavaScript 'constants.js' file.
            // UH: {
            //     attack: 2
            // },
            UtriumHydride => Boost::Attack(2.0),
            // UH2O: {
            //     attack: 3
            // },
            UtriumAcid => Boost::Attack(3.0),
            // XUH2O: {
            //     attack: 4
            // }
            CatalyzedUtriumAcid => Boost::Attack(4.0),
            // UO: {
            //     harvest: 3
            // },
            UtriumOxide => Boost::Harvest(3.0),
            // UHO2: {
            //     harvest: 5
            // },
            UtriumAlkalide => Boost::Harvest(5.0),
            // XUHO2: {
            //     harvest: 7
            // },
            CatalyzedUtriumAlkalide => Boost::Harvest(7.0),
            // KH: {
            //     capacity: 2
            // },
            KeaniumHydride => Boost::Carry(2.0),
            // KH2O: {
            //     capacity: 3
            // },
            KeaniumAcid => Boost::Carry(3.0),
            // XKH2O: {
            //     capacity: 4
            // }
            CatalyzedKeaniumAcid => Boost::Carry(4.0),
            // KO: {
            //     rangedAttack: 2,
            //     rangedMassAttack: 2
            // },
            KeaniumOxide => Boost::RangedAttack(2.0),
            // KHO2: {
            //     rangedAttack: 3,
            //     rangedMassAttack: 3
            // },
            KeaniumAlkalide => Boost::RangedAttack(4.0),
            // XKHO2: {
            //     rangedAttack: 4,
            //     rangedMassAttack: 4
            // }
            CatalyzedKeaniumAlkalide => Boost::RangedAttack(4.0),
            // LH: {
            //     build: 1.5,
            //     repair: 1.5
            // },
            LemergiumHydride => Boost::BuildAndRepair(1.5),
            // LH2O: {
            //     build: 1.8,
            //     repair: 1.8
            // },
            LemergiumAcid => Boost::BuildAndRepair(1.8),
            // XLH2O: {
            //     build: 2,
            //     repair: 2
            // },
            CatalyzedLemergiumAcid => Boost::BuildAndRepair(2.0),
            // LO: {
            //     heal: 2,
            //     rangedHeal: 2
            // },
            LemergiumOxide => Boost::Heal(2.0),
            // LHO2: {
            //     heal: 3,
            //     rangedHeal: 3
            // },
            LemergiumAlkalide => Boost::Heal(3.0),
            // XLHO2: {
            //     heal: 4,
            //     rangedHeal: 4
            // }
            CatalyzedLemergiumAlkalide => Boost::Heal(4.0),
            // ZH: {
            //     dismantle: 2
            // },
            ZynthiumHydride => Boost::Dismantle(2.0),
            // ZH2O: {
            //     dismantle: 3
            // },
            ZynthiumAcid => Boost::Dismantle(3.0),
            // XZH2O: {
            //     dismantle: 4
            // },
            CatalyzedZynthiumAcid => Boost::Dismantle(4.0),
            // ZO: {
            //     fatigue: 2
            // },
            ZynthiumOxide => Boost::Move(2.0),
            // ZHO2: {
            //     fatigue: 3
            // },
            ZynthiumAlkalide => Boost::Move(3.0),
            // XZHO2: {
            //     fatigue: 4
            // }
            CatalyzedZynthiumAlkalide => Boost::Move(4.0),
            // GH: {
            //     upgradeController: 1.5
            // },
            GhodiumHydride => Boost::UpgradeController(1.5),
            // GH2O: {
            //     upgradeController: 1.8
            // },
            GhodiumAcid => Boost::UpgradeController(1.8),
            // XGH2O: {
            //     upgradeController: 2
            // }
            CatalyzedGhodiumAcid => Boost::UpgradeController(2.0),
            // GO: {
            //     damage: .7
            // },
            GhodiumOxide => Boost::Tough(0.7),
            // GHO2: {
            //     damage: .5
            // },
            GhodiumAlkalide => Boost::Tough(0.5),
            // XGHO2: {
            //     damage: .3
            // }
            CatalyzedGhodiumAlkalide => Boost::Tough(0.3),
            // non-boost resources
            _ => return None,
        };
        Some(boost)
    }

    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s).map_err(|_| {
            D::Error::invalid_value(
                Unexpected::Str(&s),
                &"a known constant string in RESOURCES_ALL",
            )
        })
    }
}

js_deserializable!(ResourceType);

/// Translates market resource types which can include both `RESOURCE_*`
/// and `INTERSHARD_RESOURCES` constants.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MarketResourceType {
    Resource(ResourceType),
    IntershardResource(IntershardResourceType),
}

impl MarketResourceType {
    /// Helper function for deserializing from a string rather than a fake
    /// integer value.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;

        ResourceType::from_str(&s)
            .map(|ty| MarketResourceType::Resource(ty))
            .or(IntershardResourceType::from_str(&s)
                .map(|ty| MarketResourceType::IntershardResource(ty)))
            .map_err(|_| {
                D::Error::invalid_value(
                    Unexpected::Str(&s),
                    &"a known constant string in RESOURCES_ALL or INTERSHARD_RESOURCES",
                )
            })
    }
}

impl<'de> Deserialize<'de> for MarketResourceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let resource = u16::deserialize(deserializer)?;
        let resource_type = match resource {
            1 => MarketResourceType::Resource(ResourceType::Energy),
            2 => MarketResourceType::Resource(ResourceType::Power),
            3 => MarketResourceType::Resource(ResourceType::Hydrogen),
            4 => MarketResourceType::Resource(ResourceType::Oxygen),
            5 => MarketResourceType::Resource(ResourceType::Utrium),
            6 => MarketResourceType::Resource(ResourceType::Lemergium),
            7 => MarketResourceType::Resource(ResourceType::Keanium),
            8 => MarketResourceType::Resource(ResourceType::Zynthium),
            9 => MarketResourceType::Resource(ResourceType::Catalyst),
            10 => MarketResourceType::Resource(ResourceType::Ghodium),
            11 => MarketResourceType::Resource(ResourceType::Hydroxide),
            12 => MarketResourceType::Resource(ResourceType::ZynthiumKeanite),
            13 => MarketResourceType::Resource(ResourceType::UtriumLemergite),
            14 => MarketResourceType::Resource(ResourceType::UtriumHydride),
            15 => MarketResourceType::Resource(ResourceType::UtriumOxide),
            16 => MarketResourceType::Resource(ResourceType::KeaniumHydride),
            17 => MarketResourceType::Resource(ResourceType::KeaniumOxide),
            18 => MarketResourceType::Resource(ResourceType::LemergiumHydride),
            19 => MarketResourceType::Resource(ResourceType::LemergiumOxide),
            20 => MarketResourceType::Resource(ResourceType::ZynthiumHydride),
            21 => MarketResourceType::Resource(ResourceType::ZynthiumOxide),
            22 => MarketResourceType::Resource(ResourceType::GhodiumHydride),
            23 => MarketResourceType::Resource(ResourceType::GhodiumOxide),
            24 => MarketResourceType::Resource(ResourceType::UtriumAcid),
            25 => MarketResourceType::Resource(ResourceType::UtriumAlkalide),
            26 => MarketResourceType::Resource(ResourceType::KeaniumAcid),
            27 => MarketResourceType::Resource(ResourceType::KeaniumAlkalide),
            28 => MarketResourceType::Resource(ResourceType::LemergiumAcid),
            29 => MarketResourceType::Resource(ResourceType::LemergiumAlkalide),
            30 => MarketResourceType::Resource(ResourceType::ZynthiumAcid),
            31 => MarketResourceType::Resource(ResourceType::ZynthiumAlkalide),
            32 => MarketResourceType::Resource(ResourceType::GhodiumAcid),
            33 => MarketResourceType::Resource(ResourceType::GhodiumAlkalide),
            34 => MarketResourceType::Resource(ResourceType::CatalyzedUtriumAcid),
            35 => MarketResourceType::Resource(ResourceType::CatalyzedUtriumAlkalide),
            36 => MarketResourceType::Resource(ResourceType::CatalyzedKeaniumAcid),
            37 => MarketResourceType::Resource(ResourceType::CatalyzedKeaniumAlkalide),
            38 => MarketResourceType::Resource(ResourceType::CatalyzedLemergiumAcid),
            39 => MarketResourceType::Resource(ResourceType::CatalyzedLemergiumAlkalide),
            40 => MarketResourceType::Resource(ResourceType::CatalyzedZynthiumAcid),
            41 => MarketResourceType::Resource(ResourceType::CatalyzedZynthiumAlkalide),
            42 => MarketResourceType::Resource(ResourceType::CatalyzedGhodiumAcid),
            43 => MarketResourceType::Resource(ResourceType::CatalyzedGhodiumAlkalide),
            44 => MarketResourceType::Resource(ResourceType::Ops),
            45 => MarketResourceType::Resource(ResourceType::Silicon),
            46 => MarketResourceType::Resource(ResourceType::Metal),
            47 => MarketResourceType::Resource(ResourceType::Biomass),
            48 => MarketResourceType::Resource(ResourceType::Mist),
            49 => MarketResourceType::Resource(ResourceType::UtriumBar),
            50 => MarketResourceType::Resource(ResourceType::LemergiumBar),
            51 => MarketResourceType::Resource(ResourceType::ZynthiumBar),
            52 => MarketResourceType::Resource(ResourceType::KeaniumBar),
            53 => MarketResourceType::Resource(ResourceType::GhodiumMelt),
            54 => MarketResourceType::Resource(ResourceType::Oxidant),
            55 => MarketResourceType::Resource(ResourceType::Reductant),
            56 => MarketResourceType::Resource(ResourceType::Purifier),
            57 => MarketResourceType::Resource(ResourceType::Battery),
            58 => MarketResourceType::Resource(ResourceType::Composite),
            59 => MarketResourceType::Resource(ResourceType::Crystal),
            60 => MarketResourceType::Resource(ResourceType::Liquid),
            61 => MarketResourceType::Resource(ResourceType::Wire),
            62 => MarketResourceType::Resource(ResourceType::Switch),
            63 => MarketResourceType::Resource(ResourceType::Transistor),
            64 => MarketResourceType::Resource(ResourceType::Microchip),
            65 => MarketResourceType::Resource(ResourceType::Circuit),
            66 => MarketResourceType::Resource(ResourceType::Device),
            67 => MarketResourceType::Resource(ResourceType::Cell),
            68 => MarketResourceType::Resource(ResourceType::Phlegm),
            69 => MarketResourceType::Resource(ResourceType::Tissue),
            70 => MarketResourceType::Resource(ResourceType::Muscle),
            71 => MarketResourceType::Resource(ResourceType::Organoid),
            72 => MarketResourceType::Resource(ResourceType::Organism),
            73 => MarketResourceType::Resource(ResourceType::Alloy),
            74 => MarketResourceType::Resource(ResourceType::Tube),
            75 => MarketResourceType::Resource(ResourceType::Fixtures),
            76 => MarketResourceType::Resource(ResourceType::Frame),
            77 => MarketResourceType::Resource(ResourceType::Hydraulics),
            78 => MarketResourceType::Resource(ResourceType::Machine),
            79 => MarketResourceType::Resource(ResourceType::Condensate),
            80 => MarketResourceType::Resource(ResourceType::Concentrate),
            81 => MarketResourceType::Resource(ResourceType::Extract),
            82 => MarketResourceType::Resource(ResourceType::Spirit),
            83 => MarketResourceType::Resource(ResourceType::Emanation),
            84 => MarketResourceType::Resource(ResourceType::Essence),
            1001 => {
                MarketResourceType::IntershardResource(IntershardResourceType::SubscriptionToken)
            }
            _ => {
                return Err(D::Error::invalid_value(
                    Unexpected::Unsigned(resource as u64),
                    &"a valid RESOURCES_ALL or INTERSHARD_RESOURCES type integer",
                ))
            }
        };
        Ok(resource_type)
    }
}

impl Serialize for MarketResourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MarketResourceType::Resource(ty) => ty.serialize(serializer),
            MarketResourceType::IntershardResource(ty) => ty.serialize(serializer),
        }
    }
}

/// Translates the `POWER_CLASS` constants, which are classes of power creeps
#[derive(
    Copy, Clone, Debug, Display, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr,
)]
#[repr(u8)]
#[display(style = "camelCase")]
pub enum PowerCreepClass {
    /// `"operator"`
    Operator = 1,
}

js_deserializable!(PowerCreepClass);

/// Translates the `PWR_*` constants, which are types of powers used by power
/// creeps
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum PowerType {
    GenerateOps = 1,
    OperateSpawn = 2,
    OperateTower = 3,
    OperateStorage = 4,
    OperateLab = 5,
    OperateExtension = 6,
    OperateObserver = 7,
    OperateTerminal = 8,
    DisruptSpawn = 9,
    DisruptTower = 10,
    Shield = 12,
    RegenSource = 13,
    RegenMineral = 14,
    DisruptTerminal = 15,
    OperatePower = 16,
    Fortify = 17,
    OperateController = 18,
    OperateFactory = 19,
}

js_deserializable!(PowerType);

/// Translates the `EFFECT_*` constants, which are natural effect types
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, FromPrimitive, Serialize_repr, Deserialize_repr,
)]
#[repr(u16)]
pub enum NaturalEffectType {
    Invulnerability = 1001,
    CollapseTimer = 1002,
}

js_deserializable!(NaturalEffectType);

/// Translates effect types which can include both `PWR_*` and `EFFECT_*`
/// constants.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EffectType {
    PowerEffect(PowerType),
    NaturalEffect(NaturalEffectType),
}

impl<'de> Deserialize<'de> for EffectType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let effect = u16::deserialize(deserializer)?;
        let effect_type = match effect {
            1 => EffectType::PowerEffect(PowerType::GenerateOps),
            2 => EffectType::PowerEffect(PowerType::OperateSpawn),
            3 => EffectType::PowerEffect(PowerType::OperateTower),
            4 => EffectType::PowerEffect(PowerType::OperateStorage),
            5 => EffectType::PowerEffect(PowerType::OperateLab),
            6 => EffectType::PowerEffect(PowerType::OperateExtension),
            7 => EffectType::PowerEffect(PowerType::OperateObserver),
            8 => EffectType::PowerEffect(PowerType::OperateTerminal),
            9 => EffectType::PowerEffect(PowerType::DisruptSpawn),
            10 => EffectType::PowerEffect(PowerType::DisruptTower),
            12 => EffectType::PowerEffect(PowerType::Shield),
            13 => EffectType::PowerEffect(PowerType::RegenSource),
            14 => EffectType::PowerEffect(PowerType::RegenMineral),
            15 => EffectType::PowerEffect(PowerType::DisruptTerminal),
            16 => EffectType::PowerEffect(PowerType::OperatePower),
            17 => EffectType::PowerEffect(PowerType::Fortify),
            18 => EffectType::PowerEffect(PowerType::OperateController),
            19 => EffectType::PowerEffect(PowerType::OperateFactory),
            1001 => EffectType::NaturalEffect(NaturalEffectType::Invulnerability),
            1002 => EffectType::NaturalEffect(NaturalEffectType::CollapseTimer),
            _ => {
                return Err(D::Error::invalid_value(
                    Unexpected::Unsigned(effect as u64),
                    &"a valid PWR_* or EFFECT_* type integer",
                ))
            }
        };

        Ok(effect_type)
    }
}
