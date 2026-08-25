use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::spell::Spell;

#[derive(Tsify, Serialize, Deserialize)]
pub struct DurationFilter {
    // TODO: Add duration type ? HOW ??
    concentration: Option<bool>,
}

impl DurationFilter {
    pub fn accepts(&self, spell: &Spell) -> bool {
        self.concentration.map_or(true, |concentration| {
            concentration == spell.duration.concentration
        })
    }
}
