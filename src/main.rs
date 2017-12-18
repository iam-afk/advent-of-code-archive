extern crate tokio_core;
extern crate futures;
extern crate hyper;

use std::env;
use std::error::Error;
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
    with_day_input(2017, 18, |input| input.to_string(), |_| "not implemented").unwrap();
}

#[derive(Debug)]
struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: String) -> ParseError {
        ParseError { message: message }
    }
}

#[derive(Debug, PartialEq)]
enum Op {
    Reg(char),
    Val(i64),
}

impl FromStr for Op {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(x) => Ok(Op::Val(x)),
            Err(_) if s.len() == 1 => {
                match s.chars().next() {
                    Some(x) => Ok(Op::Reg(x)),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::new(format!("bad register: {}", s))),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Inst {
    Set(char, Op),
}

impl FromStr for Inst {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s.split_whitespace();
        match v.next() {
            Some("set") => {
                v.next()
                    .ok_or(ParseError::new(format!("expected first argument: {}", s)))
                    .and_then(|arg| arg.parse::<Op>())
                    .and_then(|v| if let Op::Reg(x) = v {
                        Ok(x)
                    } else {
                        Err(ParseError::new(
                            format!("first argument should be a register: {}", s),
                        ))
                    })
                    .and_then(|x| {
                        Ok((
                            x,
                            v.next().ok_or(ParseError::new(
                                format!("expected second argument: {}", s),
                            ))?,
                        ))
                    })
                    .and_then(|(x, arg)| Ok((x, arg.parse::<Op>()?)))
                    .and_then(|(x, y)| Ok(Inst::Set(x, y)))
            }
            Some(_) => unimplemented!(),
            None => Err(ParseError::new("instruction expected".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(Inst::Set('a', Op::Val(1)), "set a 1".parse().unwrap());
    }

}
