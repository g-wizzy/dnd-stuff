use std::{collections::HashSet, mem::discriminant};

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::spell::{Source, Spell};

#[derive(Tsify, Serialize, Deserialize)]
pub struct SourceFilter {
    sources: HashSet<Source>,
}

impl SourceFilter {
    pub fn accepts(&self, spell: &Spell) -> bool {
        self.sources
            .iter()
            .any(|source| discriminant(source) == discriminant(&spell.source))
    }
}
