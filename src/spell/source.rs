use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::errors::ParseError;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
#[wasm_bindgen]
pub enum Source {
    PHB,
    ForgottenRealms,
    Eberron,
}

impl Source {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        match text {
            "Player´s Handbook 2024 (BR)" | "Player´s Handbook 2024" => Ok(Self::PHB),
            "Eberron: Forge of the Artificer" => Ok(Self::Eberron),
            "Forgotten Realms: Heroes of Faerûn" => Ok(Self::ForgottenRealms),
            _ => Err(ParseError(format!("invalid source (french): '{}'", text))),
        }
    }
}

impl From<Source> for u32 {
    fn from(value: Source) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for Source {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Source::PHB),
            1 => Ok(Source::ForgottenRealms),
            2 => Ok(Source::Eberron),
            _ => Err(ParseError(format!(
                "Invalid discriminant for Source: {}",
                value
            ))),
        }
    }
}
