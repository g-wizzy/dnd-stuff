use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::spell::Spell;

#[derive(Tsify, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LevelFilter {
    pub min: u8,
    pub max: u8,
}

impl LevelFilter {
    pub fn accepts(&self, spell: &Spell) -> bool {
        self.min <= spell.level && spell.level <= self.max
    }
}
