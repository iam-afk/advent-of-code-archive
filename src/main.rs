extern crate tokio_core;
extern crate futures;
extern crate hyper;

use std::env;
use std::error::Error;

use tokio_core::reactor::Core;

use hyper::{Client, Method};
use hyper::client::Request;
use hyper::header::Cookie;

use futures::future::Future;
use futures::Stream;

fn with_day_input<F, T>(year: u32, day: u32, f: F) -> Result<(), Box<Error>>
where
    F: Fn(&str) -> T,
    T: std::fmt::Display,
{
    let session = env::args().nth(1).expect(
        "Specify session token as first argument",
    );
    let uri = format!("http://adventofcode.com/{}/day/{}/input", year, day)
        .parse()?;

    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let mut cookie = Cookie::new();
    cookie.append("session", session);
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(cookie);
    let fetch = client.request(req).and_then(|res| res.body().concat2());
    core.run(fetch).map(|chunk| {
        let body = String::from_utf8_lossy(&chunk);
        let answer = f(&body);
        println!("{}", answer);
    })?;
    Ok(())
}

fn main() {
    with_day_input(2017, 2, |input| {
        input.lines().map(|row| checksum(row)).sum::<u32>()
    }).unwrap();
}

fn checksum(s: &str) -> u32 {
    let numbers: Vec<_> = s.trim()
        .split_whitespace()
        .map(|e| e.parse::<u32>().unwrap())
        .collect();
    numbers.iter().max().unwrap() - numbers.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(8, checksum("5 1 9 5"));
        assert_eq!(4, checksum("7 5 3"));
        assert_eq!(6, checksum("2 4 6 8"));
    }

}
