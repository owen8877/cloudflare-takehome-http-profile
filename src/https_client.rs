use openssl::ssl::{SslConnector, SslMethod};
use std::net::TcpStream;
use std::io::{Read, Write};

fn find_first_blank_string(arr: &[&str]) -> Option<usize> {
    for (i, s) in arr.iter().enumerate() {
        if s.len() == 0 {
            return Some(i);
        }
    }
    None
}

pub(crate) struct HTTPSResponse {
    header: Vec<String>,
    body: String,
}

pub(crate) struct HTTPSClient {
    connector: SslConnector,
}

fn url_extractor(url: String) -> (String, String) {
    match url.find('/') {
        None => {
            // No slashes, so url is just the domain and the path is '/'
            (url, "/".to_string())
        }
        Some(i) => {
            let (domain, path) = url.split_at(i);
            (domain.to_string(), path.to_string())
        }
    }
}

impl HTTPSClient {
    pub(crate) fn new() -> Self {
        Self {
            connector: SslConnector::builder(SslMethod::tls()).unwrap().build(),
        }
    }

    pub(crate) fn get(&self, url: String) -> HTTPSResponse {
        let (domain, path) = url_extractor(url);
        let stream = TcpStream::connect(format!("{}:443", domain)).unwrap();
        let mut stream = self.connector.connect(domain.as_str(), stream).unwrap();

        let request = format!("GET {} HTTP/1.0\r\nHost: {}\r\nAccept: */*\r\n\r\n", path, domain);
        debug!("Request:\n{}", request);
        stream.write_all(request.as_bytes()).unwrap();
        let mut res = vec![];
        stream.read_to_end(&mut res).unwrap();
        let response_with_header = String::from_utf8_lossy(&res);
        debug!("Raw response:\n {}", response_with_header);
        let mut response_arr: Vec<&str> = response_with_header.split("\r\n").collect();
        let split_index = find_first_blank_string(response_arr.as_slice()).expect("Cannot understand HTTP response!");
        let mut body = response_arr.split_off(split_index);
        body.remove(0);
        let header = response_arr;

        debug!("Header (len={}):\n{:?}", header.len(), header);
        debug!("Body (len={}):\n{:?}", body.len(), body);

        HTTPSResponse {
            header: header.into_iter().map(|s| s.to_string()).collect(),
            body: body[0].to_string(),
        }
    }
}