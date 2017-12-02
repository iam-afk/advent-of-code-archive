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
    let v: Vec<_> = s.chars().map(|c| c as u32 - '0' as u32).collect();
    let step = v.len() / 2;
    for i in 0..step {
        if v[i] == v[i + step] {
            answer += 2 * v[i];
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(6, sum_of_match_next("1212"));
        assert_eq!(0, sum_of_match_next("1221"));
        assert_eq!(4, sum_of_match_next("123425"));
        assert_eq!(12, sum_of_match_next("123123"));
        assert_eq!(4, sum_of_match_next("12131415"));
    }

}
