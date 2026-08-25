use std::panic;

use wasm_bindgen::prelude::*;

pub mod errors;
pub mod spell;
pub mod spellbook;

mod utils;

#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

// #[wasm_bindgen]
// pub fn get_spell(index: usize) -> Option<Spell> {
//     if let Some(sb) = &SPELLBOOK {
//         match sb.spells.get(index) {
//             Some(spell) => Some(spell.clone()),
//             None => None,
//         }
//     } else {
//         None
//     }
// }
//
// #[wasm_bindgen]
// pub fn get_spells(filter: SpellFilter) -> Vec<Spell> {
//     if let Some(sb) = &SPELLBOOK {
//         sb.filter(&filter).map(|spell| spell.clone()).collect()
//     } else {
//         vec![]
//     }
// }

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND: &str = r#"
export interface PLuginApi {
    init(config: Record<string, unknown>): Promise<void>;
    process(data: Uint8Array): Promise<Uint8Array>;
    destroy(): void;
}
"#;
