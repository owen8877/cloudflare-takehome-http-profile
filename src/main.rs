#[macro_use]
extern crate log;
extern crate openssl;
extern crate regex;

use std::error::Error;

use https_client::HTTPSClient;

mod https_client;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let client = HTTPSClient::new();

    client.get("raw.githubusercontent.com/owen8877/calendar-as-diary/master/.gitignore".to_string());

    Ok(())
}