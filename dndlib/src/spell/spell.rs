use std::{collections::HashSet, fs::read_to_string, path::PathBuf};

use regex::Regex;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde::Serialize;
use tsify::Tsify;
use unidecode::unidecode;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::spell::ActionCost;
use crate::spell::Class;
use crate::spell::Duration;
use crate::spell::MagicSchool;
use crate::spell::Range;
use crate::spell::Source;
use crate::spell::SpellComponents;

use crate::errors::ParseError;

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
pub struct Spell {
    pub normalized_name: String,
    pub name: String,
    pub level: u8,
    pub action_cost: ActionCost,
    pub school: MagicSchool,
    pub classes: HashSet<Class>,
    pub source: Source,
    pub components: SpellComponents,
    pub range: Range,
    pub duration: Duration,
    pub description: String,
}

impl Spell {
    pub fn read_from_dir(base_dir: &PathBuf) -> Vec<Self> {
        let mut result = Vec::<Spell>::new();

        for entry in base_dir.read_dir().expect("read_dir failed") {
            if let Ok(entry) = entry {
                if let Ok(html) = read_to_string(entry.path()) {
                    match Spell::from_html(&html) {
                        Ok(spell) => {
                            result.push(spell);
                        }
                        Err(error) => {
                            println!(
                                "Error parsing HTML in file {:?}: {}",
                                entry.file_name(),
                                error
                            );
                        }
                    }
                } else {
                    println!("Could not read file");
                }
            }
        }
        result
    }

    pub fn from_html(html: &str) -> Result<Self, ParseError> {
        let doc = Html::parse_document(html);

        // NAME
        let name_selector = Selector::parse("h1").unwrap();
        let name = doc
            .select(&name_selector)
            .next()
            .ok_or("Could not find the name")?
            .inner_html();

        // LEVEL, MAGIC SCHOOL, CLASSES
        let level_school_classes_selector = Selector::parse("div.ecole").unwrap();
        let level_school_class = doc
            .select(&level_school_classes_selector)
            .next()
            .ok_or("Could not find level, school, and classes")?
            .inner_html();
        let (level, school, classes) = parse_level_school_classes(&level_school_class)?;

        // ACTION COST
        let action_cost_selector = Selector::parse("div.t").unwrap();
        let action_cost_str = doc
            .select(&action_cost_selector)
            .next()
            .ok_or("Could not find action cost")?
            .inner_html();
        let action_cost = ActionCost::from_french(&action_cost_str)?;

        // RANGE
        let range_selector = Selector::parse("div.r").unwrap();
        let range_str = doc
            .select(&range_selector)
            .next()
            .ok_or("Could not find range")?
            .inner_html();
        let range = Range::from_french(&range_str)?;

        // DURATION
        let duration_selector = Selector::parse("div.d").unwrap();
        let duration_str = doc
            .select(&duration_selector)
            .next()
            .ok_or("Could not find duration")?
            .inner_html();
        let duration = Duration::from_french(&duration_str)?;

        // COMPONENTS
        let components_selector = Selector::parse("div.c").unwrap();
        let components_str = doc
            .select(&components_selector)
            .next()
            .ok_or("Could not find components")?
            .inner_html();
        let components = SpellComponents::from_french(&components_str)?;

        // SOURCE
        let source_selector = Selector::parse("div.source").unwrap();
        let source_str = doc
            .select(&source_selector)
            .next()
            .ok_or("Could not find source")?
            .inner_html();
        let source = Source::from_french(&source_str)?;

        // DESCRIPTION
        let description_selector = Selector::parse("div.description").unwrap();
        let description = match doc.select(&description_selector).next() {
            Some(element) => element.inner_html(),
            None => "Description unavailable".to_string(),
        };

        let normalized_name = normalize_string(&name);
        Ok(Spell {
            normalized_name,
            name,
            level,
            action_cost,
            school,
            classes,
            source,
            components,
            range,
            duration,
            description,
        })
    }
}

fn parse_level_school_classes(html: &str) -> Result<(u8, MagicSchool, HashSet<Class>), ParseError> {
    let level_school_class_regex = Regex::new(r"(\w+) de niveau (\d) \((.*)\)").unwrap();
    return match level_school_class_regex.captures(&html) {
        Some(captures) => {
            let school_str = captures.get(1).unwrap().as_str();
            let level_str = captures.get(2).unwrap().as_str();
            let classes_str = captures.get(3).unwrap().as_str();
            let classes_vec: Result<Vec<_>, _> =
                classes_str.split(", ").map(Class::from_french).collect();
            Ok((
                level_str.parse::<u8>().or(Err(ParseError(format!(
                    "cannot parse level: '{}'",
                    level_str
                ))))?,
                MagicSchool::from_french(school_str)?,
                HashSet::<Class>::from_iter(classes_vec?),
            ))
        }
        None => Err(ParseError(format!(
            "malformed level-school-classes string: '{}'",
            html
        ))),
    };
}

fn normalize_string(text: &str) -> String {
    unidecode(text)
        .replace(" ", "_")
        .replace("'", "_")
        .to_lowercase()
}
