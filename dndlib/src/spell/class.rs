use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::errors::ParseError;

#[derive(Tsify, Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Class {
    Bard,
    Cleric,
    Druid,
    Paladin,
    Ranger,
    Sorcerer,
    Warlock,
    Wizard,
    Artificer,
}

impl Class {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        match text {
            "Barde" => Ok(Self::Bard),
            "Clerc" => Ok(Self::Cleric),
            "Druide" => Ok(Self::Druid),
            "Paladin" => Ok(Self::Paladin),
            "Rôdeur" => Ok(Self::Ranger),
            "Ensorceleur" => Ok(Self::Sorcerer),
            "Occultiste" => Ok(Self::Warlock),
            "Magicien" => Ok(Self::Wizard),
            "Artificer" => Ok(Self::Artificer),
            &_ => Err(ParseError(format!("Unknown class '{}' (french)", text))),
        }
    }
}
