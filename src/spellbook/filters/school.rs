use std::{collections::HashSet, mem::discriminant};

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::spell::{MagicSchool, Spell};

#[derive(Tsify, Serialize, Deserialize)]
pub struct MagicSchoolFilter {
    schools: HashSet<MagicSchool>,
}

impl MagicSchoolFilter {
    pub fn accepts(&self, spell: &Spell) -> bool {
        self.schools
            .iter()
            .any(|school| discriminant(school) == discriminant(&spell.school))
    }
}
