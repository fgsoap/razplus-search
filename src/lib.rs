use clap::Parser;
use scraper::{Html, Selector};
use std::fmt::Write as _;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, forbid_empty_values = true, validator = validate_name)]
    name: Vec<String>,
    #[clap(short, long, parse(from_occurrences))]
    verbosity: usize,
}

fn validate_name(name: &str) -> Result<(), String> {
    if name.trim().len() != name.len() {
        Err(String::from("name cannot have leading and trailing space"))
    } else {
        Ok(())
    }
}

pub async fn run() -> Result<String, reqwest::Error> {
    eprintln!("{:?}", Args::parse());

    let mut results = String::new();
    for name in Args::parse().name {
        let url = format!(
            "https://www.raz-plus.com/search/ajax-search.html?doSearch=Search&searchTerms={}",
            name
        );
        eprintln!("Searching {:?}\n", url);

        let res = reqwest::get(url).await?;
        let body = res.text().await?;

        let document = Html::parse_document(&body);
        let top_selector = Selector::parse(r#"div.info>h2>a"#).unwrap();
        let inner_elector = Selector::parse(r#"div.info>div.category"#).unwrap();

        let mut result = String::new();
        for (top_index, top_element) in document.select(&top_selector).enumerate() {
            for (inner_index, inner_element) in document.select(&inner_elector).enumerate() {
                if top_index == inner_index
                    && top_element.html().contains("leveled-books")
                    && inner_element.html().contains("leveled-books")
                {
                    if let Err(e) = writeln!(
                        result,
                        "{}{}",
                        top_element
                            .html()
                            .split("\n                            ")
                            .collect::<Vec<_>>()[0..3]
                            .iter()
                            .cloned()
                            .collect::<String>()
                            .replace("href=\"/books/leveled-books/book/?", "")
                            .replace("amp;", "")
                            .replace("  ", "")
                            .replace("<strong>", "")
                            .replace("</strong>", ""),
                        inner_element.html().split(" Level ").collect::<Vec<_>>()[1]
                            .replace("                    </div>", "")
                            .replace("<strong>", "")
                            .replace("</strong>", "")
                    ) {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        if let Err(e) = writeln!(results, "{}", result) {
            eprintln!("{}", e);
            std::process::exit(1);
        };
    }
    Ok(results)
}
