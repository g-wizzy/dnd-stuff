use std::panic;

use lazy_static::lazy_static;
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

pub mod spell;
use spell::Spell;

use crate::spellbook::{SpellBook, SpellFilter};

pub mod spellbook;

pub mod errors;

mod utils;

lazy_static! {
    static ref spells: SpellBook = SpellBook::build();
}

#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn get_spell(index: usize) -> Option<Ts<Spell>> {
    match spells.spells.get(index) {
        Some(spell) => Some(spell.into_ts().unwrap()),
        None => None,
    }
}

#[wasm_bindgen]
pub fn get_spells(filter: Ts<SpellFilter>) -> Vec<Ts<Spell>> {
    spells
        .filter(&filter.to_rust().unwrap())
        .map(|spell| spell.into_ts().unwrap())
        .collect()
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND: &str = r#"
export interface PLuginApi {
    init(config: Record<string, unknown>): Promise<void>;
    process(data: Uint8Array): Promise<Uint8Array>;
    destroy(): void;
}
"#;
