use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::errors::ParseError;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
#[wasm_bindgen]
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

impl From<Class> for u32 {
    fn from(value: Class) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for Class {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Class::Bard),
            1 => Ok(Class::Cleric),
            2 => Ok(Class::Druid),
            3 => Ok(Class::Paladin),
            4 => Ok(Class::Ranger),
            5 => Ok(Class::Sorcerer),
            6 => Ok(Class::Warlock),
            7 => Ok(Class::Wizard),
            8 => Ok(Class::Artificer),
            _ => Err(ParseError(format!(
                "Invalid discriminant for Class: {}",
                value
            ))),
        }
    }
}
