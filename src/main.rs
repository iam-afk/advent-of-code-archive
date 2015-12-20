extern crate regex;

use std::io;
use std::collections::HashMap;

use regex::*;

use Source::*;
use Op::*;

#[derive(Debug)]
enum Source {
    Wire(String),
    Signal(u16)
}

#[derive(Debug)]
enum Op {
    Direct(Source),
    Not(Source),
    And(Source, Source),
    Or(Source, Source),
    LShift(Source, u16),
    RShift(Source, u16)
}

fn parse(token: &str) -> Source {
    if is_match(r"\d+", token).unwrap() {
        Source::Signal(token.parse::<u16>().unwrap())
    } else {
        Source::Wire(token.to_string())
    }
}

fn get(wires: &HashMap<String, Op>, wire: &str) -> u16 {
    let mut cache = HashMap::new();

    fn calc(wires: &HashMap<String, Op>, wire: &str, cache: &mut HashMap<String, u16>) -> u16 {
        if cache.contains_key(wire) {
            return *cache.get(wire).unwrap();
        }
        let signal = match wires.get(wire).unwrap() {
            &Direct(ref source) => f(source, wires, cache),
            &Not(ref source) => !f(source, wires, cache),
            &And(ref lhs, ref rhs) => f(lhs, wires, cache) & f(rhs, wires, cache),
            &Or(ref lhs, ref rhs) => f(lhs, wires, cache) | f(rhs, wires, cache),
            &LShift(ref source, shift) => f(source, wires, cache) << shift,
            &RShift(ref source, shift) => f(source, wires, cache) >> shift
        };
        cache.insert(wire.to_string(), signal);
        signal
    }

    fn f(source: &Source, wires: &HashMap<String, Op>, cache: &mut HashMap<String, u16>) -> u16 {
        match source {
            &Signal(signal) => signal,
            &Wire(ref wire) => calc(wires, wire, cache)
        }
    }

    calc(wires, wire, &mut cache)
}

fn main() {
    let stdin = io::stdin();

    let mut wires = HashMap::new();

    let mut line = String::new();
    while stdin.read_line(&mut line).unwrap() > 0 {
        {
            let v: Vec<&str> = line.trim().split(' ').collect();
            match v[1] {
                "AND" =>
                    wires.insert(v[4].to_string(), Op::And(parse(v[0]), parse(v[2]))),
                "OR" =>
                    wires.insert(v[4].to_string(), Op::Or(parse(v[0]), parse(v[2]))),
                "LSHIFT" =>
                    wires.insert(v[4].to_string(), Op::LShift(parse(v[0]), v[2].parse::<u16>().unwrap())),
                "RSHIFT" =>
                    wires.insert(v[4].to_string(), Op::RShift(parse(v[0]), v[2].parse::<u16>().unwrap())),
                "->" =>
                    wires.insert(v[2].to_string(), Op::Direct(parse(v[0]))),
                _ =>
                    wires.insert(v[3].to_string(), Op::Not(parse(v[1])))
            };
        }
        line.clear();
    }
    println!("{:?}", get(&wires, "a"));
}
