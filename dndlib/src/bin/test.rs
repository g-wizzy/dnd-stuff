use dndlib::spellbook::{SpellBook, SpellFilter};

fn main() {
    let spellbook = SpellBook::build();
    let filter = SpellFilter {
        search: Some("telepa".to_string()),
        level: None,
        action_type: None,
        ritual: None,
        school: None,
        class: None,
        source: None,
        component_verbal: None,
        component_somatic: None,
        component_material: None,
        range: None,
        duration_type: None,
        concentration: None,
    };

    for spell in spellbook.filter(&filter) {
        println!("{:?}", spell);
    }
}
