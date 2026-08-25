use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::spell::Spell;

#[derive(Tsify, Serialize, Deserialize)]
pub struct ComponentsFilter {
    verbal: Option<bool>,
    somatic: Option<bool>,
    material: Option<bool>,
}

impl ComponentsFilter {
    pub fn accepts(&self, spell: &Spell) -> bool {
        self.verbal
            .map_or(true, |verbal| verbal == spell.components.verbal)
            && self
                .somatic
                .map_or(true, |somatic| somatic == spell.components.somatic)
            && self.material.map_or(true, |material| {
                material == spell.components.material.is_some()
            })
    }
}
