use std::{collections::HashSet, mem::discriminant};

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::spell::{Class, Spell};

#[derive(Tsify, Serialize, Deserialize)]
pub struct ClassFilter {
    classes: HashSet<Class>,
}

impl ClassFilter {
    pub fn accepts(&self, spell: &Spell) -> bool {
        self.classes.iter().any(|filter_class| {
            spell
                .classes
                .iter()
                .any(|spell_class| discriminant(spell_class) == discriminant(filter_class))
        })
    }
}
