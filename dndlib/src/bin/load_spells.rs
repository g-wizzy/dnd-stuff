use clap::Parser;
use scraper::{Html, Selector};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use dndlib::spell::Spell;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL of the list of spells that will be parsed and crawled
    #[arg(long)]
    url: Option<String>,
}

const HTMLS_LOCATION: &str = "htmls";

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let htmls_dir = PathBuf::from(HTMLS_LOCATION);

    if let Some(url) = args.url {
        download_htmls(&url, &htmls_dir).await;
    } else if !htmls_dir.exists() {
        panic!("Must use --url argument if no spells have been downloaded yet.")
    }

    let spells = Spell::read_from_dir(&htmls_dir);
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

async fn download_htmls(url: &str, dir: &PathBuf) {
    fs::create_dir_all(&dir).unwrap();
    let html = reqwest::get(url).await.unwrap().text().await.unwrap();
    let doc = Html::parse_document(&html);

    let selector = Selector::parse("td.item>a").unwrap();
    for link in doc.select(&selector) {
        let name = link.attr("href").unwrap().split("/").last().unwrap();
        let spell_url = format!("{url}/{name}");
        let mut spell_file = File::create(dir.join(name)).unwrap();
        spell_file
            .write_all(
                reqwest::get(&spell_url)
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
    }
}
