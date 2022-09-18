use clap::Parser;
use scraper::{Html, Selector};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let name = Args::parse().name;
    let url = format!(
        "https://www.raz-plus.com/search/ajax-search.html?doSearch=Search&searchTerms={}",
        name
    );
    eprintln!("Fetching {:?}\n", url);

    let res = reqwest::get(url).await?;
    let body = res.text().await?;

    let document = Html::parse_document(&body);
    let top_selector = Selector::parse(r#"div.info>h2>a"#).unwrap();
    let inner_elector = Selector::parse(r#"div.info>div.category"#).unwrap();

    for (top_index, top_element) in document.select(&top_selector).enumerate() {
        for (inner_index, inner_element) in document.select(&inner_elector).enumerate() {
            if top_index == inner_index && top_element.html().contains("leveled-books") {
                println!(
                    "{:#?}",
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
                        + &inner_element.html().split(" words, ").collect::<Vec<_>>()[1]
                            .replace("                    </div>", "")
                );
            }
        }
    }

    // println!("{}", body);

    Ok(())
}
