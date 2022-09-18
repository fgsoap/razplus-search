use clap::Parser;

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
    println!("{}", body);

    Ok(())
}
