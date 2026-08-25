use std::{collections::HashSet, mem::discriminant};

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::spell::{ActionType, Spell};

#[derive(Tsify, Serialize, Deserialize)]
pub struct ActionFilter {
    action_types: HashSet<ActionType>,
    ritual: Option<bool>,
}

impl ActionFilter {
    pub fn accepts(&self, spell: &Spell) -> bool {
        self.action_types.iter().any(|action_type| {
            discriminant(action_type) == discriminant(&spell.action_cost.action_type)
        }) && self
            .ritual
            .map_or(true, |ritual| ritual == spell.action_cost.ritual)
    }
}
