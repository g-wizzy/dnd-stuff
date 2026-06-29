use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::errors::ParseError;

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
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
