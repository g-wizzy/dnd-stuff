use clap::Parser;
use std::{fs, path::PathBuf};

use dndlib::spell::Spell;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path (absolute or relative) to the directory containing the html files
    #[arg()]
    path: PathBuf,
}

pub fn main() {
    let args = Args::parse();
    let spells = Spell::read_from_dir(&args.path);

    let spells_ron = ron::ser::to_string(&spells).unwrap();

    fs::write(
        "src/spell/serialized.rs",
        format!(
            r##" pub fn get_spells_ron() -> &'static str {{
    r#"{}"#
}}"##,
            spells_ron
        ),
    )
    .unwrap()
}
