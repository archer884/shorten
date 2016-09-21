extern crate grabinput;
extern crate hyper;
extern crate rand;
extern crate url;

use hyper::Client;
use rand::Rng;
use std::io::Read;
use url::percent_encoding::{PercentEncode, QUERY_ENCODE_SET};

fn main() {
    let urls = grabinput::from_args().with_fallback();
    let client = Client::new();

    let mut results: Vec<_> = urls
        .map(|url| client.get(&format_request(url.trim())).send())
        .map(|result| result.map(|ref mut res| read_response(res)))
        .collect();

    rand::thread_rng().shuffle(&mut results);

    for result in &results {
        match *result {
            Err(ref e) => println!("error: {}", e),
            Ok(ref url) => println!("{}", url),
        }
    }
}

fn format_request(url: &str) -> String {
    let url = encode_url(url);
    format!("https://is.gd/create.php?format=simple&url={}", url)
}

#[allow(unused)]
fn read_response<T: Read>(response: &mut T) -> String {
    let mut buffer = String::new();
    response.read_to_string(&mut buffer);
    buffer
} 

fn encode_url(url: &str) -> PercentEncode<QUERY_ENCODE_SET> {
    url::percent_encoding::percent_encode(url.as_bytes(), url::percent_encoding::QUERY_ENCODE_SET)
}
