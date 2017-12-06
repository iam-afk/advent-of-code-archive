extern crate tokio_core;
extern crate futures;
extern crate hyper;

use std::env;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

use tokio_core::reactor::Core;

use hyper::{Client, Method};
use hyper::client::Request;
use hyper::header::Cookie;

use futures::future::Future;
use futures::Stream;

fn with_day_input<F1, T1, F2, T2>(year: u32, day: u32, f1: F1, f2: F2) -> Result<(), Box<Error>>
where
    F1: Fn(&str) -> T1,
    F2: Fn(&str) -> T2,
    T1: std::fmt::Display,
    T2: std::fmt::Display,
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
    let (answer1, answer2) = core.run(fetch).map(|chunk| {
        let body = String::from_utf8_lossy(&chunk);
        (f1(&body), f2(&body))
    })?;
    println!(" *: {}\n**: {}", answer1, answer2);
    Ok(())
}

fn main() {
    with_day_input(
        2017,
        5,
        |input| {
            let mut offsets: Vec<_> = input
                .lines()
                .map(|line| line.trim().parse().unwrap())
                .collect();
            steps(&mut offsets)
        },
        |_| "not implemented",
    ).unwrap();
}

#[derive(Debug)]
enum Jump {
    Previous,
    Next,
}

use Jump::*;

#[derive(Debug)]
struct Offset {
    jump: Jump,
    value: usize,
}

impl FromStr for Offset {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let offset = match s.chars().next() {
            Some('-') => Offset {
                jump: Previous,
                value: s[1..].parse()?,
            },
            _ => Offset {
                jump: Next,
                value: s.parse()?,
            },
        };
        Ok(offset)
    }
}

fn steps(offsets: &mut [Offset]) -> usize {
    let len = offsets.len();
    let mut pos = 0usize;
    let mut count = 0usize;
    loop {
        count += 1;
        let offset = &mut offsets[pos];
        let next = match offset.jump {
            Previous => pos.checked_sub(offset.value),
            Next => pos.checked_add(offset.value),
        };
        pos = match next {
            Some(value) if value < len => {
                match offset.jump {
                    Previous => offset.value -= 1,
                    Next => offset.value += 1,
                }
                if offset.value == 0 {
                    offset.jump = Next;
                }
                value
            }
            _ => break,
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            5,
            steps(
                &mut [
                    Offset {
                        jump: Next,
                        value: 0,
                    },
                    Offset {
                        jump: Next,
                        value: 3,
                    },
                    Offset {
                        jump: Next,
                        value: 0,
                    },
                    Offset {
                        jump: Next,
                        value: 1,
                    },
                    Offset {
                        jump: Previous,
                        value: 3,
                    },
                ],
            )
        );
    }

}
