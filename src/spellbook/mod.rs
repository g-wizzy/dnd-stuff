mod filter;
mod filters;

pub use filter::SpellFilter;
use tsify::Ts;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::spell::{Spell, get_spells_ron};

#[derive(Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct SpellBook {
    pub spells: Vec<Spell>,
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
impl SpellBook {
    pub fn build() -> Self {
        let mut spells = ron::de::from_str::<Vec<Spell>>(&get_spells_ron()).unwrap();
        spells.sort_by_key(|spell| spell.name.clone());
        spells.sort_by_key(|spell| spell.level);

        Self { spells }
    }
    pub fn filter(&self, filter: &Ts<SpellFilter>) -> Vec<Spell> {
        log(&format!("{:?}", filter.js_value()));
        let filter_rust = filter.to_rust();
        if let Err(error) = filter_rust {
            log(&error.to_string());
        }
        self.spells
            .iter()
            .filter(move |spell| filter.to_rust().unwrap().accepts(spell))
            .map(|spell| spell.clone())
            .collect()
    }
}
