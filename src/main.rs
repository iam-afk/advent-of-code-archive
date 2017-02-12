use std::env;
use std::io::prelude::*;
use std::io::BufReader;

extern crate hyper;

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::header::Cookie;

fn fetch_input() -> BufReader<Response> {
    let mut cookie = String::from("session=");
    cookie.push_str(&env::args().nth(1).expect("Specify session token as first argument"));
    let client = Client::new();
    let response = client.get("http://adventofcode.com/2016/day/8/input")
        .header(Cookie(vec![cookie]))
        .send()
        .unwrap();
    BufReader::new(response)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}

}
