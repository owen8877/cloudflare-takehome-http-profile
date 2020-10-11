#[macro_use]
extern crate log;

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::time::Instant;

use getopts::Options;
use https_client::HTTPSClient;
use regex::Regex;

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
            let re = Regex::new(r"HTTP/1.[01] ([\d]+)").unwrap();
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
        Err(_e) => {}
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

    if let Some((url, request_n)) = get_opt() {
        let mut profiles = vec![];
        for i in 0..request_n {
            let profile = cycle_wrapper(&client, url.clone());
            debug!("Cycle {}: {:?}", i, profile);
            profiles.push(profile);
        }
        analyze(&profiles);
    }

    Ok(())
}

fn analyze(profiles: &[ProfileData]) {
    let n = profiles.len();
    if n == 0 {
        println!("No profiles to analyze.");
        return;
    }

    let mut times: Vec<u128> = profiles.iter().map(|p| p.time).collect();
    times.sort();
    let min_time = times[0];
    let max_time = times[n - 1];
    let median_time = times[n / 2];
    let mean_time = times.iter().fold(0, |acc, x| acc + x) / n as u128;

    let success_requests = profiles.iter().filter(|x| x.request_success).count();
    let success_ratio = success_requests * 100 / n;
    let error_codes: HashSet<u32> = profiles.iter().map(|p| p.code).collect();
    let mut error_codes: Vec<&u32> = error_codes.iter().filter(|&&x| x > 0 && !(200 <= x && x < 400)).collect();
    error_codes.sort();

    let mut sizes: Vec<u32> = profiles.iter().map(|p| p.size).collect();
    sizes.sort();
    let min_size = sizes[0];
    let max_size = sizes[n - 1];

    println!("Summary:");
    println!("Requests made:");
    println!("    Total    {}", n);
    println!("    Success  {}", success_requests);
    println!("    Ratio    {}%", success_ratio);
    println!("Size(bytes) per request:");
    println!("    Smallest {}", min_size);
    println!("    Largest  {}", max_size);
    println!("Time(ms) per request:");
    println!("    Fastest  {}", min_time);
    println!("    Mean     {}", mean_time);
    println!("    Median   {}", median_time);
    println!("    Slowest  {}", max_time);
    println!("Error code encountered: {:?}", error_codes);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn get_opt() -> Option<(String, u32)> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("u", "url", "the target to fetch", "https://...");
    opts.optopt("p", "profile", "the number of requests", "[int]");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(&program, opts);
            panic!(f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return None;
    }
    let url = match matches.opt_str("u") {
        Some(url) => url,
        None => {
            print_usage(&program, opts);
            return None;
        }
    };
    let request_n = matches.opt_get::<u32>("p")
        .expect("Cannot parse arguments --profile")
        .unwrap_or(1);
    Some((url, request_n))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_analyze() {
        let profiles = vec![
            ProfileData { code: 200, parse_success: true, request_success: true, size: 10, time: 100 },
            ProfileData { code: 200, parse_success: true, request_success: true, size: 100, time: 100 },
            ProfileData { code: 200, parse_success: true, request_success: true, size: 1000, time: 100 },
            ProfileData { code: 200, parse_success: true, request_success: true, size: 20, time: 200 },
            ProfileData { code: 200, parse_success: true, request_success: true, size: 10, time: 400 },
            ProfileData { code: 200, parse_success: true, request_success: true, size: 42, time: 1600 },
            ProfileData { code: 200, parse_success: true, request_success: true, size: 76, time: 100 },
            ProfileData { code: 400, parse_success: true, request_success: false, size: 76, time: 500 },
            ProfileData { code: 401, parse_success: true, request_success: false, size: 176, time: 3500 },
            ProfileData { code: 402, parse_success: true, request_success: false, size: 276, time: 1500 },
            ProfileData { code: 403, parse_success: true, request_success: false, size: 376, time: 2500 },
            ProfileData { code: 304, parse_success: true, request_success: false, size: 376, time: 2500 },
            ProfileData { code: 0, parse_success: false, request_success: false, size: 376, time: 2500 },
        ];
        analyze(&profiles);
    }
}