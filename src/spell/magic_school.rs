use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::errors::ParseError;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
#[wasm_bindgen]
pub enum MagicSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

impl MagicSchool {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        match text {
            "Abjuration" => Ok(Self::Abjuration),
            "Invocation" => Ok(Self::Conjuration),
            "Divination" => Ok(Self::Divination),
            "Enchantement" => Ok(Self::Enchantment),
            "Évocation" => Ok(Self::Evocation),
            "Illusion" => Ok(Self::Illusion),
            "Nécromancie" => Ok(Self::Necromancy),
            "Transmutation" => Ok(Self::Transmutation),
            &_ => Err(ParseError(format!(
                "unknown magic school '{}' (french)",
                text
            ))),
        }
    }
}

impl From<MagicSchool> for u32 {
    fn from(value: MagicSchool) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for MagicSchool {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MagicSchool::Abjuration),
            1 => Ok(MagicSchool::Conjuration),
            2 => Ok(MagicSchool::Divination),
            3 => Ok(MagicSchool::Enchantment),
            4 => Ok(MagicSchool::Evocation),
            5 => Ok(MagicSchool::Illusion),
            6 => Ok(MagicSchool::Necromancy),
            7 => Ok(MagicSchool::Transmutation),
            _ => Err(ParseError(format!(
                "Invalid discriminant for MagicSchool: {}",
                value
            ))),
        }
    }
}
