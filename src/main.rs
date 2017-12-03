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
    with_day_input(2017, 3, |input| {
        let data: i32 = input.trim().parse().unwrap();
        steps(data)
    }).unwrap();
}

fn steps(data: i32) -> i32 {
    if data == 1 {
        return 0;
    }
    let mut first: i32;
    let mut last = 1;
    let mut step = 1;
    loop {
        first = last + 1;
        last += 4 * 2 * step;
        if last >= data {
            break;
        }
        step += 1;
    }
    let part = (data - first) / (2 * step);
    let middle = first + 2 * step * part + step - 1;
    step + (middle - data).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(0, steps(1));
        assert_eq!(3, steps(12));
        assert_eq!(2, steps(23));
        assert_eq!(31, steps(1024));
    }

}
