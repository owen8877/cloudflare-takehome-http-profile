extern crate env_logger;
extern crate getopt_long;
#[macro_use]
extern crate log;
extern crate openssl;
extern crate regex;

use std::error::Error;
use std::time::Instant;

use regex::Regex;

use https_client::HTTPSClient;

mod https_client;

#[derive(Debug)]
struct ProfileData {
    code: u32,
    parse_success: bool,
    request_success: bool,
    size: u32,
    time: u128,
}

fn cycle_wrapper(client: &HTTPSClient, url: String) -> ProfileData {
    let start = Instant::now();

    let mut parse_success = false;
    let mut request_success = false;
    let mut size = 0;
    let mut code = 0;
    match client.get(url) {
        Ok(response) => {
            parse_success = true;
            let re = Regex::new(r"HTTP/1.[01] ([\d].*) OK").unwrap();
            for header in &response.header {
                match re.captures(header) {
                    Some(cap) => {
                        code = cap[1].parse().unwrap();
                        break;
                    }
                    None => continue
                }
            }
            request_success = 200 <= code && code < 400;
            size = response.body.as_bytes().len() as u32;
        }
        Err(_e) => {

        }
    }

    ProfileData {
        code,
        parse_success,
        request_success,
        size,
        time: start.elapsed().as_millis(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let client = HTTPSClient::new();

    let url = "https://linktree-style-website.xdroid.workers.dev/".to_string();
    let profile = cycle_wrapper(&client, url);
    println!("{:?}", profile);

    Ok(())
}