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

macro_rules! parse_error {
    ($msg:expr, $s:ident) => (ParseError { message: format!("{}: {}", $msg, $s) });
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
            _ => Err(parse_error!("bad register", s)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Inst {
    Snd(Op),
    Set(char, Op),
    Add(char, Op),
    Mul(char, Op),
    Mod(char, Op),
    Rcv(Op),
    Jgz(Op, isize),
}

macro_rules! inst {
    ($inst:ident, val, $comp:expr, $s:ident) => (
        $comp.next()
            .ok_or(parse_error!("expected argument", $s))
            .and_then(|arg| arg.parse::<Op>())
            .and_then(|x| Ok(Inst::$inst(x)))
    );
    ($inst:ident, reg + val, $comp:expr, $s:ident) => (
        $comp.next()
            .ok_or(parse_error!("expected first argument", $s))
            .and_then(|arg| match arg.parse::<Op>()? {
                Op::Reg(x) => Ok(x),
                _ => Err(parse_error!("first argument should be a register", $s)),
            })
            .and_then(|x| {
                $comp.next()
                    .ok_or(parse_error!("expected second argument", $s))
                    .and_then(|arg| arg.parse::<Op>())
                    .and_then(|y| Ok(Inst::$inst(x, y)))
            })
    );
    ($inst:ident, reg + offset, $comp:expr, $s:ident) => (
        $comp.next()
            .ok_or(parse_error!("expected first argument", $s))
            .and_then(|arg| arg.parse::<Op>())
            .and_then(|x| {
                $comp.next()
                    .ok_or(parse_error!("expected second argument", $s))
                    .and_then(|arg| arg.parse::<isize>().map_err(|_| parse_error!("offset expected", $s)))
                    .and_then(|y| Ok(Inst::$inst(x, y)))
            })
    );
}

impl FromStr for Inst {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s.split_whitespace();
        match v.next() {
            Some("snd") => inst!(Snd, val, v, s),
            Some("set") => inst!(Set, reg + val, v, s),
            Some("add") => inst!(Add, reg + val, v, s),
            Some("mul") => inst!(Mul, reg + val, v, s),
            Some("mod") => inst!(Mod, reg + val, v, s),
            Some("rcv") => inst!(Rcv, val, v, s),
            Some("jgz") => inst!(Jgz, reg + offset, v, s),
            Some(_) => unimplemented!(),
            None => Err(parse_error!("instruction expected".to_string(), s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(Inst::Snd(Op::Reg('a')), "snd a".parse().unwrap());
        assert_eq!(Inst::Set('a', Op::Val(1)), "set a 1".parse().unwrap());
        assert_eq!(Inst::Add('a', Op::Val(2)), "add a 2".parse().unwrap());
        assert_eq!(Inst::Mul('a', Op::Reg('a')), "mul a a".parse().unwrap());
        assert_eq!(Inst::Mod('a', Op::Val(5)), "mod a 5".parse().unwrap());
        assert_eq!(Inst::Rcv(Op::Reg('a')), "rcv a".parse().unwrap());
        assert_eq!(Inst::Jgz(Op::Reg('a'), -1), "jgz a -1".parse().unwrap());
    }

}
