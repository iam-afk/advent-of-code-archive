extern crate tokio_core;
extern crate futures;
extern crate hyper;

use std::env;

use tokio_core::reactor::Core;

use hyper::{Client, Method};
use hyper::client::Request;
use hyper::header::Cookie;

use futures::future::Future;
use futures::Stream;

fn main() {
    let session = env::args().nth(1).expect(
        "Specify session token as first argument",
    );
    let uri = "http://adventofcode.com/2017/day/2/input".parse().unwrap();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let mut cookie = Cookie::new();
    cookie.append("session", session);
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(cookie);
    let fetch = client.request(req).and_then(|res| res.body().concat2());
    core.run(fetch)
        .map(|chunk| {
            let body = String::from_utf8_lossy(&chunk);

            let input: Vec<&str> = body.lines().collect();
            println!("{}", checksum(&input));
        })
        .unwrap();
}

fn checksum(s: &[&str]) -> u32 {
    let mut answer = 0u32;
    for row in s {
        let numbers: Vec<_> = row.trim()
            .split_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect();
        answer += numbers.iter().max().unwrap() - numbers.iter().min().unwrap();
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(18, checksum(&vec!["5 1 9 5", "7 5 3", "2 4 6 8"]));
    }

}
