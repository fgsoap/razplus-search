#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let result = razplus_search::run().await?;
    println!("{}", result);
    Ok(())
}
