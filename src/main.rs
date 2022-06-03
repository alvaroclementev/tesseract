extern crate dotenv;
extern crate egg_mode;

use dotenv::dotenv;
use egg_mode::Token;
use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

#[derive(Debug)]
enum TessError {
    IoError(std::io::Error),
    TwitterAPIError(egg_mode::error::Error),
}

impl From<std::io::Error> for TessError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<egg_mode::error::Error> for TessError {
    fn from(e: egg_mode::error::Error) -> Self {
        Self::TwitterAPIError(e)
    }
}

impl Display for TessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TessError::IoError(e) => write!(f, "IoError: {}", e),
            TessError::TwitterAPIError(e) => write!(f, "TwitterAPIError: {}", e),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), TessError> {
    dotenv().ok();

    let twitter_api_key =
        std::env::var("TWITTER_API_KEY").expect("TWITTER_API_KEY env var should be set");
    let twitter_api_key_secret = std::env::var("TWITTER_API_KEY_SECRET")
        .expect("TWITTER_API_KEY_SECRET env var should be set");

    // let (token, user_id, screen_name) =
    //     user_pin_auth(twitter_api_key, twitter_api_key_secret).await?;
    // println!(
    //     "Logged in as user {} ({} - {:?})",
    //     &screen_name, &user_id, &token
    // );

    let token = bearer_auth(twitter_api_key, twitter_api_key_secret).await?;

    let timeline =
        egg_mode::tweet::user_timeline("rustlang", false, true, &token).with_page_size(5);

    let (_timeline, feed) = timeline.start().await?;
    for tweet in feed.response {
        println!();
        println!("{:?}", &tweet);
    }

    Ok(())
}

async fn bearer_auth(api_key: String, api_key_secret: String) -> Result<Token, TessError> {
    // Let's try to get a list of the last tweets
    let con_token = egg_mode::KeyPair::new(api_key, api_key_secret);
    // Get a bearer token
    let bearer_token = egg_mode::auth::bearer_token(&con_token).await?;

    Ok(bearer_token)
}

/// Use the PIN Authentication flow get an access token for use on one session
async fn user_pin_auth(
    api_key: String,
    api_key_secret: String,
) -> Result<(Token, u64, String), TessError> {
    // Let's try to get a list of the last tweets
    let con_token = egg_mode::KeyPair::new(api_key, api_key_secret);
    // With `oob` we use PIN-based authentication, which is the recomended one
    // when you cannot easily redirect the user to "login with twitter", i.e:
    // in a CLI app
    let request_token = egg_mode::auth::request_token(&con_token, "oob")
        .await
        .expect("should be able to get a request token");
    let auth_url = egg_mode::auth::authorize_url(&request_token);

    // Give the auth url to the user, they can sign in to twitter and accept
    // the app's permissions. They'll receive a PIN in return, which they need
    // to input here:
    println!("Go to this link to get an authentication PIN: {}", auth_url);
    let user_pin = prompt_user("Insert PIN")?;

    // Get an actual access token that we can use to call the Twitter API
    // endpoints
    let (token, user_id, screen_name) =
        egg_mode::auth::access_token(con_token, &request_token, user_pin)
            .await
            .expect("must be able to login");

    Ok((token, user_id, screen_name))
}

fn prompt_user(prompt: &str) -> Result<String, TessError> {
    let mut s = String::new();
    print!("{} > ", prompt);
    stdout().flush()?;
    stdin().read_line(&mut s).expect("must be a valid string");
    Ok(s.trim().to_string())
}
