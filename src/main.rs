extern crate dotenv;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let twitter_api_key =
        std::env::var("TWITTER_API_KEY").expect("TWITTER_API_KEY env var should be set");
    let twitter_api_key_secret =
        std::env::var("TWITTER_API_KEY_SECRET").expect("TWITTER_API_KEY_SECRET env var should be set");
    println!("{} ({})", &twitter_api_key, &twitter_api_key_secret);
}
