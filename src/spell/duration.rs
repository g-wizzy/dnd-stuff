use regex::Regex;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::errors::ParseError;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Duration {
    pub duration_type: DurationType,
    pub duration: Option<TimedDuration>,
    pub concentration: bool,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
#[wasm_bindgen]
pub enum DurationType {
    Instantaneous,
    Special,
    Dispelled,
    DispelledOrTriggered,
    Timed,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum TimeUnit {
    Day,
    Hour,
    Minute,
    Round,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct TimedDuration {
    value: u8,
    unit: TimeUnit,
}

impl Duration {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        let regex = Regex::new(r"<strong>Durée </strong>: (Concentration, )?(.*)").unwrap();
        let captures = regex.captures(text).unwrap();
        let concentration = captures.get(1).is_some();

        match captures.get(2).unwrap().as_str() {
            "Spéciale" => Ok(Self {
                duration_type: DurationType::Special,
                duration: None,
                concentration,
            }),
            "Jusqu'à dissipation" => Ok(Self {
                duration_type: DurationType::Dispelled,
                duration: None,
                concentration,
            }),
            "Jusqu'à dissipation ou déclenchement" => Ok(Self {
                duration_type: DurationType::DispelledOrTriggered,
                duration: None,
                concentration,
            }),
            "Instantanée" => Ok(Self {
                duration_type: DurationType::Instantaneous,
                duration: None,
                concentration,
            }),
            _ => {
                let regex = Regex::new(r"(\d+) (round|minute|heure|jour)s?").unwrap();
                if let Some(captures) = regex.captures(text) {
                    Ok(Self {
                        duration_type: DurationType::Timed,
                        duration: Some(TimedDuration {
                            value: captures.get(1).unwrap().as_str().parse().unwrap(),
                            unit: TimeUnit::from_french(captures.get(2).unwrap().as_str())?,
                        }),
                        concentration,
                    })
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

impl From<DurationType> for u32 {
    fn from(value: DurationType) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for DurationType {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DurationType::Instantaneous),
            1 => Ok(DurationType::Special),
            2 => Ok(DurationType::Dispelled),
            3 => Ok(DurationType::DispelledOrTriggered),
            4 => Ok(DurationType::Timed),
            _ => Err(ParseError(format!(
                "Invalid discriminant for DurationType: {}",
                value
            ))),
        }
    }
}
