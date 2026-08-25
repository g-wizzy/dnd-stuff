use regex::Regex;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::errors::ParseError;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct MaterialComponent {
    pub text: String,
    // pub value: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct SpellComponents {
    pub verbal: bool,
    pub somatic: bool,
    pub material: Option<MaterialComponent>,
}

impl SpellComponents {
    pub fn from_french(text: &str) -> Result<Self, ParseError> {
        let materials_regex = Regex::new(r"M \((.*)\)$").unwrap();
        let material_component = if let Some(captures) = materials_regex.captures(&text) {
            Some(MaterialComponent {
                text: captures.get(1).unwrap().as_str().to_string(),
            })
        } else {
            None
        };

        Ok(Self {
            verbal: text.contains('V'),
            somatic: text.contains('S'),
            material: material_component,
        })
    }
}
