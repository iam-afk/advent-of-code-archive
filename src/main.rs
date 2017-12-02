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
            for line in body.lines() {
                println!("{:?}", line);
            }
        })
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}

}
