mod filter;

pub use filter::SpellFilter;

use crate::spell::{Spell, get_spells_ron};

pub struct SpellBook {
    pub spells: Vec<Spell>,
}

impl SpellBook {
    pub fn build() -> Self {
        Self {
            spells: ron::de::from_str(&get_spells_ron()).unwrap(),
        }
    }
    pub fn filter(&self, filter: &SpellFilter) -> impl Iterator<Item = &Spell> {
        self.spells
            .iter()
            .filter(move |spell| filter.accepts(spell))
    }

    // TODO: Add sorting
}
