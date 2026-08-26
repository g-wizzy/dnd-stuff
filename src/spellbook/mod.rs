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
impl SpellBook {
    pub fn build() -> Self {
        let mut spells = ron::de::from_str::<Vec<Spell>>(&get_spells_ron()).unwrap();
        spells.sort_by_key(|spell| spell.name.clone());
        spells.sort_by_key(|spell| spell.level);

        Self { spells }
    }
    pub fn filter(&self, filter: &Ts<SpellFilter>) -> Vec<Spell> {
        self.spells
            .iter()
            .filter(move |spell| filter.to_rust().unwrap().accepts(spell))
            .map(|spell| spell.clone())
            .collect()
    }
}
