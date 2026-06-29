use regex::Regex;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::errors::ParseError;

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction,
    Long(String),
}

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
pub struct ActionCost {
    pub action_type: ActionType,
    pub ritual: bool,
    pub additional_info: Option<String>,
}

impl ActionCost {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        let regex = Regex::new(r"<strong>Temps d'incantation </strong>: (Action bonus|Action|Réaction|\d+ (minutes?|heures?))(.*?)( ou Rituel)?$").unwrap();
        match regex.captures(&text) {
            Some(captures) => {
                let action_type_str = captures.get(1).unwrap().as_str();
                let additional_info = captures.get(3).unwrap().as_str();
                let ritual = captures.get(4).is_some();

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
            ActionType::Long(text.to_string())
        }
    }
}
