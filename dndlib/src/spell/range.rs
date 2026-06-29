use regex::Regex;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::errors::ParseError;

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
pub enum Area {
    Cone { length: u8 },
    Cube { side: u8 },
    Emanation { radius: u8 },
    Line { length: u8 },
}

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
pub struct Distance {
    pub value: u8,
    pub large_unit: bool, // true if km instead of m, or miles instead of feet
}

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
pub enum Range {
    Self_,
    Touch,
    Sight,
    Special,
    Unlimited,
    Distance(Distance),
}

impl Range {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        let regex = Regex::new(r"<strong>Portée </strong>: (.*)").unwrap();
        match regex.captures(&text) {
            Some(captures) => match captures.get(1).unwrap().as_str() {
                "Personnelle" => Ok(Self::Self_),
                "Contact" => Ok(Self::Touch),
                "Vue" => Ok(Self::Sight),
                "Spéciale" => Ok(Self::Special),
                "Illimitée" => Ok(Self::Unlimited),
                other => Ok(Self::Distance(Distance::from_french(other)?)),
            },
            None => Err(ParseError(format!(
                "malformed range html (french): '{}'",
                text
            ))),
        }
    }
}

impl Distance {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        let distance_regex = Regex::new(r"(\d+)(,50?)? (m|km)").unwrap();
        match distance_regex.captures(text) {
            Some(captures) => {
                let mut base_distance_str = captures.get(1).unwrap().as_str().to_string();
                let has_point_five = captures.get(2).is_some();
                if has_point_five {
                    base_distance_str.push_str("50");
                } else {
                    base_distance_str.push_str("00");
                }
                let value = base_distance_str.parse::<u32>().unwrap() / 150u32;
                let large_unit = captures.get(3).unwrap().as_str() == "km";

                Ok(Self {
                    value: value as u8,
                    large_unit,
                })
            }
            None => Err(ParseError(format!(
                "malformed distance (french): '{}'",
                text
            ))),
        }
    }
}
