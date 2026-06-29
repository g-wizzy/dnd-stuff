use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::errors::ParseError;

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
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
