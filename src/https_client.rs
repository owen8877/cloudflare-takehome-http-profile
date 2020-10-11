use std::error::Error;
use std::fmt::Formatter;
use std::fmt;
use std::io::{Read, Write};
use std::net::TcpStream;

use openssl::ssl::{SslConnector, SslMethod};
use regex::Regex;

fn find_first_blank_string(arr: &[&str]) -> Option<usize> {
    for (i, s) in arr.iter().enumerate() {
        if s.len() == 0 {
            return Some(i);
        }
    }
    None
}

pub(crate) struct HTTPSResponse {
    pub(crate) header: Vec<String>,
    pub(crate) body: String,
}

pub(crate) struct HTTPSClient {
    connector: SslConnector,
}

fn url_extractor(url: String) -> (String, String) {
    // First get rid of http:// and https:// things.
    let re = Regex::new(r"(https://|http://)").unwrap();
    let url = re.replace(url.as_str(), "");

    // Then match the pattern.
    match url.find('/') {
        None => {
            // No slashes, so url is just the domain and the path is '/'
            (url.to_string(), "/".to_string())
        }
        Some(i) => {
            let (domain, path) = url.split_at(i);
            (domain.to_string(), path.to_string())
        }
    }
}

#[derive(Debug)]
struct ParseError {
    response: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot parse response: {}", self.response)
    }
}

impl ParseError {
    fn new(response: String) -> Self {
        Self {
            response,
        }
    }
}

impl Error for ParseError {}

impl HTTPSClient {
    pub(crate) fn new() -> Self {
        Self {
            connector: SslConnector::builder(SslMethod::tls()).unwrap().build(),
        }
    }

    pub(crate) fn get(&self, url: String) -> Result<HTTPSResponse, Box<dyn Error>> {
        let (domain, path) = url_extractor(url);
        let stream = TcpStream::connect(format!("{}:443", domain)).unwrap();
        let mut stream = self.connector.connect(domain.as_str(), stream).unwrap();

        let request = format!("GET {} HTTP/1.0\r\nHost: {}\r\nAccept: */*\r\n\r\n", path, domain);
        debug!("Request:\n{}", request);
        stream.write_all(request.as_bytes()).unwrap();
        let mut res = vec![];
        stream.read_to_end(&mut res).unwrap();
        let response_raw = String::from_utf8_lossy(&res);
        debug!("Raw response:\n{}", response_raw);
        let mut response_arr: Vec<&str> = response_raw.split("\r\n").collect();
        let split_index = find_first_blank_string(response_arr.as_slice()).ok_or(ParseError::new(response_raw.to_string()))?;
        let mut body = response_arr.split_off(split_index);
        body.remove(0);
        let header = response_arr;

        debug!("Header (len={}):\n{:?}", header.len(), header);
        debug!("Body (len={}):\n{:?}", body.len(), body);

        Ok(HTTPSResponse {
            header: header.into_iter().map(|s| s.to_string()).collect(),
            body: body[0].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_extractor() {
        assert_eq!(url_extractor("abc.efg.com".to_string()), ("abc.efg.com".to_string(), "/".to_string()));
        assert_eq!(url_extractor("http://abc.efg.com".to_string()), ("abc.efg.com".to_string(), "/".to_string()));
        assert_eq!(url_extractor("https://abc.efg.com".to_string()), ("abc.efg.com".to_string(), "/".to_string()));
        assert_eq!(url_extractor("abc.efg.com/1".to_string()), ("abc.efg.com".to_string(), "/1".to_string()));
        assert_eq!(url_extractor("http://abc.efg.com/1".to_string()), ("abc.efg.com".to_string(), "/1".to_string()));
        assert_eq!(url_extractor("https://abc.efg.com/1".to_string()), ("abc.efg.com".to_string(), "/1".to_string()));
        assert_eq!(url_extractor("abc.efg.com/1/2/".to_string()), ("abc.efg.com".to_string(), "/1/2/".to_string()));
        assert_eq!(url_extractor("http://abc.efg.com/1/2/".to_string()), ("abc.efg.com".to_string(), "/1/2/".to_string()));
        assert_eq!(url_extractor("https://abc.efg.com/1/2/".to_string()), ("abc.efg.com".to_string(), "/1/2/".to_string()));
    }

    #[test]
    fn test_client_get() {
        let client = HTTPSClient::new();
        let url = "http://feeds.bbci.co.uk/news/world/rss.xml".to_string();
        match client.get(url) {
            Ok(response) => {
                println!("Header: {:?}", response.header);
                println!("Body: {}", response.body);
            }
            Err(e) => println!("Error raised: {}", e),
        }
    }
}