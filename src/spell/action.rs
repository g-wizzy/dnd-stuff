use regex::Regex;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::errors::ParseError;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
#[wasm_bindgen]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction,
    Long,
}

impl From<ActionType> for u32 {
    fn from(value: ActionType) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for ActionType {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ActionType::Action),
            1 => Ok(ActionType::BonusAction),
            2 => Ok(ActionType::Reaction),
            3 => Ok(ActionType::Long),
            _ => Err(ParseError(format!(
                "Invalid discriminant for ActionType: {value}"
            ))),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct ActionCost {
    pub action_type: ActionType,
    pub ritual: bool,
    pub additional_info: Option<String>,
}

#[wasm_bindgen]
impl ActionCost {
    pub fn to_french(&self) -> String {
        let mut result: String = match self.action_type {
            ActionType::Action => "Action",
            ActionType::BonusAction => "Action bonus",
            ActionType::Reaction => "Réaction",
            ActionType::Long => "",
        }
        .into();

        if let Some(additional_info) = &self.additional_info {
            if result.len() > 0 {
                result.push_str(&format!(", {additional_info}"));
            } else {
                result.push_str(&additional_info);
            }
        }

        result
    }
}

impl ActionCost {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        let regex = Regex::new(r"<strong>Temps d'incantation </strong>: (Action bonus|Action|Réaction|\d+ (minutes?|heures?))(.*?)( ou Rituel)?$").unwrap();
        match regex.captures(&text) {
            Some(captures) => {
                let action_type_str = captures.get(1).unwrap().as_str();
                let mut additional_info = captures.get(3).unwrap().as_str().to_string();
                let ritual = captures.get(4).is_some();

                let action_type = ActionType::from_french(action_type_str);
                if matches!(action_type, ActionType::Long) {
                    additional_info = format!("{action_type_str}, {additional_info}");
                }

                Ok(Self {
                    action_type: ActionType::from_french(action_type_str),
                    ritual,
                    additional_info: if additional_info.is_empty() {
                        None
                    } else {
                        Some(additional_info.to_string())
                    },
                })
            }
            None => Err(ParseError(format!(
                "malformed action html (french): '{}'",
                text
            ))),
        }
    }
}

impl ActionType {
    pub fn from_french(text: &str) -> Self {
        if text.starts_with("Action bonus") {
            ActionType::BonusAction
        } else if text.starts_with("Action") {
            ActionType::Action
        } else if text.starts_with("Réaction") {
            ActionType::Reaction
        } else {
            ActionType::Long
        }
    }
}
