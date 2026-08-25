use dndlib::spellbook::{SpellBook, SpellFilter};

use tsify::Ts;

fn main() {
    let spellbook = SpellBook::build();
    let filter = SpellFilter {
        search: Some("telepa".to_string()),
        level: None,
        action: None,
        duration: None,
        components: None,
        source: None,
        class: None,
        school: None,
    };

    for spell in spellbook.filter(&Ts::from_rust(&filter).unwrap()) {
        println!("{:?}", spell);
    }
}
