use serde::{Deserialize, Serialize};
use tsify::Tsify;
use unidecode::unidecode;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    spell::Spell,
    spellbook::filters::{
        ActionFilter, ClassFilter, ComponentsFilter, DurationFilter, LevelFilter,
        MagicSchoolFilter, SourceFilter,
    },
};

#[derive(Tsify, Serialize, Deserialize)]
pub struct SpellFilter {
    pub search: Option<String>,
    pub level: Option<LevelFilter>,
    pub action: Option<ActionFilter>,
    pub school: Option<MagicSchoolFilter>,
    pub class: Option<ClassFilter>,
    pub source: Option<SourceFilter>,
    pub components: Option<ComponentsFilter>,
    // pub range: Option<Range>, TODO: Implement range filtering ? HOW ??
    pub duration: Option<DurationFilter>,
}

impl SpellFilter {
    pub fn accepts(&self, spell: &Spell) -> bool {
        self.level
            .as_ref()
            .map_or(true, |level_filter| level_filter.accepts(spell))
            && self
                .action
                .as_ref()
                .map_or(true, |action_filter| action_filter.accepts(spell))
            && self
                .school
                .as_ref()
                .map_or(true, |school_filter| school_filter.accepts(spell))
            && self
                .class
                .as_ref()
                .map_or(true, |class_filter| class_filter.accepts(spell))
            && self
                .source
                .as_ref()
                .map_or(true, |source_filter| source_filter.accepts(spell))
            && self
                .components
                .as_ref()
                .map_or(true, |comps_filters| comps_filters.accepts(spell))
            && self
                .duration
                .as_ref()
                .map_or(true, |duration_filter| duration_filter.accepts(spell))
            && self.apply_search(spell)
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
