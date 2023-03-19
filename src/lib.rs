use clap::Parser;
use colored::*;
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
    println!("{:?}", Args::parse());

    let mut results = String::new();
    for name in Args::parse().name {
        let url = format!(
            "https://www.raz-plus.com/search/ajax-search.html?doSearch=Search&searchTerms={}",
            name
        );
        eprintln!("Searching {}", url.bright_cyan());

        let res = reqwest::get(url).await?;
        let body = res.text().await?;

        let document = Html::parse_document(&body);
        let selector = Selector::parse(r#"li.resource>a"#).unwrap();

        for element in document.select(&selector) {
            let href = element.value().attr("href").unwrap();
            if href.contains("leveled-books") && href.contains("langId=1") {
                let id = href.split("?id=").collect::<Vec<_>>()[1]
                    .split("&langId=")
                    .collect::<Vec<_>>()[0];

                let fragment = Html::parse_fragment(&element.html());

                let title_selector = Selector::parse(r#"div.info>h2>div>strong"#).unwrap();
                let title = fragment
                    .select(&title_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()[0];

                let category_selector = Selector::parse(r#"div.info>div.category"#).unwrap();
                let category = fragment
                    .select(&category_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>();
                let final_category =
                    category[0].replace('\n', "").trim().to_string() + " " + category[1].trim();

                let result = id.to_owned() + " " + title + &final_category;

                if let Err(e) = writeln!(results, "{}", result) {
                    eprintln!("{}", e);
                    std::process::exit(1);
                };
            }
        }
    }
    Ok(results)
}
