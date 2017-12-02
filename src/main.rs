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
    let uri = "http://adventofcode.com/2017/day/1/input".parse().unwrap();

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

            println!("{}", &body);
            println!("{:?}", sum_of_match_next(&body.trim()));
        })
        .unwrap();
}

fn sum_of_match_next(s: &str) -> u32 {
    let mut answer = 0u32;
    let mut p = '\0';
    for c in s.chars() {
        if c == p {
            answer += c as u32 - '0' as u32;
        }
        p = c;
    }
    if s.chars().nth(0).unwrap() == p {
        answer += p as u32 - '0' as u32;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(3, sum_of_match_next("1122"));
        assert_eq!(4, sum_of_match_next("1111"));
        assert_eq!(0, sum_of_match_next("1234"));
        assert_eq!(9, sum_of_match_next("91212129"));
    }

}
