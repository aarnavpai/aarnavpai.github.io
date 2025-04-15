pub fn print_help(prog_name: &str) {
    println!(
        "Usage: {prog_name} [COMMAND]

Where COMMAND is:
    scrape    scrapes pronouns and prepositions from sources"
    )
}

const PRONOUNS_URL: &'static str = "https://www.thefreedictionary.com/List-of-pronouns.htm";
const PREPOSITIONS_URL: &'static str = "https://www.thefreedictionary.com/List-of-prepositions.htm";

fn scrape_pronouns() {
    let mut vec = Vec::new();
    let res = reqwest::blocking::get(PRONOUNS_URL)
        .unwrap()
        .text()
        .unwrap();
    let sel = scraper::Selector::parse("section#Article li").unwrap();
    let frag = scraper::Html::parse_fragment(&res);
    for li in frag.select(&sel) {
        vec.push(
            li.text().collect::<Vec<_>>()[0]
                .split_whitespace()
                .next()
                .unwrap_or(""),
        );
    }
    std::fs::write(
        "./data/pronouns.txt",
        vec.join("\n").to_lowercase().as_bytes(),
    )
    .unwrap();
}

fn scrape_prepositions() {
    let mut vec = Vec::new();
    let res = reqwest::blocking::get(PREPOSITIONS_URL)
        .unwrap()
        .text()
        .unwrap();
    let sel = scraper::Selector::parse("div.p:nth-child(34) li").unwrap();
    let frag = scraper::Html::parse_fragment(&res);
    for li in frag.select(&sel) {
        vec.push(li.text().collect::<Vec<_>>()[0]);
    }
    std::fs::write(
        "./data/prepositions.txt",
        vec.join("\n").to_lowercase().as_bytes(),
    )
    .unwrap();
}

pub fn scrape() {
    if !std::fs::exists("./data").unwrap() || !std::fs::metadata("./data").unwrap().is_dir() {
        println!("Creating directory...");
        std::fs::create_dir("./data").unwrap();
    }
    let handle1 = std::thread::spawn(|| {
        println!("Scraping pronouns...");
        scrape_pronouns();
    });
    let handle2 = std::thread::spawn(|| {
        println!("Scraping prepositions...");
        scrape_prepositions();
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let prog_name = &args[0];
    if args.len() != 2 || args.contains(&"--help".to_string()) {
        print_help(prog_name);
        std::process::exit(0);
    }
    match args[1].as_str() {
        "scrape" => scrape(),
        _ => {
            print_help(prog_name);
            std::process::exit(1);
        }
    }
}
