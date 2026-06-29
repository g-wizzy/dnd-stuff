use regex::Regex;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::errors::ParseError;

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
pub struct Duration {
    pub duration_type: DurationType,
    pub concentration: bool,
}

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
pub enum DurationType {
    Instantaneous,
    Special,
    Dispelled,
    DispelledOrTriggered,
    Timed(u8, TimeUnit),
}

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
pub enum TimeUnit {
    Day,
    Hour,
    Minute,
    Round,
}

impl Duration {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        let regex = Regex::new(r"<strong>Durée </strong>: (Concentration, )?(.*)").unwrap();
        let captures = regex.captures(text).unwrap();

        Ok(Self {
            duration_type: DurationType::from_french(captures.get(2).unwrap().as_str())?,
            concentration: captures.get(1).is_some(),
        })
    }
}

impl DurationType {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        match text {
            "Spéciale" => Ok(Self::Special),
            "Jusqu'à dissipation" => Ok(Self::Dispelled),
            "Jusqu'à dissipation ou déclenchement" => Ok(Self::DispelledOrTriggered),
            "Instantanée" => Ok(Self::Instantaneous),
            _ => {
                let regex = Regex::new(r"(\d+) (round|minute|heure|jour)s?").unwrap();
                if let Some(captures) = regex.captures(text) {
                    Ok(Self::Timed(
                        captures.get(1).unwrap().as_str().parse().unwrap(),
                        TimeUnit::from_french(captures.get(2).unwrap().as_str())?,
                    ))
                } else {
                    Err(ParseError(format!("invalid duration (french): '{}'", text)))
                }
            }
        }
    }
}

impl TimeUnit {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        match text {
            "jour" => Ok(Self::Day),
            "heure" => Ok(Self::Hour),
            "minute" => Ok(Self::Minute),
            "round" => Ok(Self::Round),
            _ => Err(ParseError(format!(
                "unknown time unit (french): '{}'",
                text
            ))),
        }
    }
}
