#[macro_use]
extern crate log;
extern crate openssl;

use https_client::HTTPSClient;

mod https_client;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let client = HTTPSClient::new();

    client.get("raw.githubusercontent.com/owen8877/calendar-as-diary/master/.gitignore".to_string());

    Ok(())
}