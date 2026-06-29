use std::mem::discriminant;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use unidecode::unidecode;

use crate::spell::{ActionType, Class, DurationType, MagicSchool, Range, Source, Spell};

#[derive(Tsify, Serialize, Deserialize)]
pub struct SpellFilter {
    pub search: Option<String>,
    pub level: Option<u8>,
    pub action_type: Option<ActionType>,
    pub ritual: Option<bool>,
    pub school: Option<MagicSchool>,
    pub class: Option<Class>,
    pub source: Option<Source>,
    pub component_verbal: Option<bool>,
    pub component_somatic: Option<bool>,
    pub component_material: Option<bool>,
    pub range: Option<Range>,
    pub duration_type: Option<DurationType>,
    pub concentration: Option<bool>,
}

impl SpellFilter {
    pub fn accepts(&self, spell: &Spell) -> bool {
        self.check_level(spell)
            && self.check_action_type(spell)
            && self.check_ritual(spell)
            && self.check_school(spell)
            && self.check_class(spell)
            && self.check_source(spell)
            && self.check_components(spell)
            && self.check_range(spell)
            && self.check_duration(spell)
            && self.check_concentration(spell)
            && self.apply_search(spell)
    }

    fn check_concentration(&self, spell: &Spell) -> bool {
        match self.concentration {
            Some(concentration) => concentration == spell.duration.concentration,
            None => true,
        }
    }

    fn check_duration(&self, spell: &Spell) -> bool {
        match &self.duration_type {
            Some(duration_type) => {
                discriminant(duration_type) == discriminant(&spell.duration.duration_type)
            }
            None => true,
        }
    }

    fn check_range(&self, spell: &Spell) -> bool {
        match &self.range {
            Some(range) => discriminant(range) == discriminant(&spell.range),
            None => true,
        }
    }

    fn check_components(&self, spell: &Spell) -> bool {
        if let Some(verbal) = self.component_verbal
            && verbal != spell.components.verbal
        {
            return false;
        }
        if let Some(somatic) = self.component_somatic
            && somatic != spell.components.somatic
        {
            return false;
        }
        if let Some(material) = self.component_material
            && material != spell.components.material.is_some()
        {
            return false;
        }
        true
    }

    fn check_source(&self, spell: &Spell) -> bool {
        match &self.source {
            Some(source) => discriminant(source) == discriminant(&spell.source),
            None => true,
        }
    }

    fn check_class(&self, spell: &Spell) -> bool {
        match &self.class {
            Some(class) => spell
                .classes
                .iter()
                .any(|any_class| discriminant(class) == discriminant(any_class)),
            None => true,
        }
    }

    fn check_school(&self, spell: &Spell) -> bool {
        match &self.school {
            Some(school) => discriminant(school) == discriminant(&spell.school),
            None => true,
        }
    }

    fn check_ritual(&self, spell: &Spell) -> bool {
        match self.ritual {
            Some(ritual) => ritual == spell.action_cost.ritual,
            None => true,
        }
    }

    fn check_action_type(&self, spell: &Spell) -> bool {
        match &self.action_type {
            Some(action_type) => {
                discriminant(action_type) == discriminant(&spell.action_cost.action_type)
            }
            None => true,
        }
    }

    fn check_level(&self, spell: &Spell) -> bool {
        match self.level {
            Some(level) => level == spell.level,
            None => true,
        }
    }

    fn apply_search(&self, spell: &Spell) -> bool {
        if let Some(search) = &self.search {
            unidecode(&search)
                .to_lowercase()
                .replace("'", " ")
                .split(" ")
                .filter(|word| word.len() > 1)
                .all(|word| spell.normalized_name.contains(word))
        } else {
            true
        }
    }
}
