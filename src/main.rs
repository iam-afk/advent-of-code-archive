use std::io;
use std::io::Read;
use std::iter::Peekable;
use std::str::Chars;

type Lexer<'a> = Peekable<Chars<'a>>;

enum Value {
    Str(String),
    Int(i64)
}

use Value::*;

fn main() {
    let mut stdin = io::stdin();
    let mut data = String::new();
    stdin.read_to_string(&mut data).unwrap();
    let mut chars = data.trim().chars().peekable();

    fn object(chars: &mut Lexer) -> i64 {
        assert!('{' == chars.next().unwrap());
        let mut sum = 0i64;
        let mut red = false;
        loop {
            match chars.peek() {
                Some(&'}') => { chars.next(); break },
                Some(&',') => { chars.next(); },
                _ => {
                    string(chars);
                    assert!(':' == chars.next().unwrap());
                    match value(chars) {
                        Int(v) => sum += v,
                        Str(ref v) if v == "red" => red = true,
                        _ => ()
                    };
                }
            };
        }
        if red { 0i64 } else { sum }
    }

    fn array(chars: &mut Lexer) -> i64 {
        assert!('[' == chars.next().unwrap());
        let mut sum = 0i64;
        loop {
            match chars.peek() {
                Some(&']') => { chars.next(); break },
                Some(&',') => { chars.next(); },
                _ => match value(chars) {
                    Int(v) => sum += v,
                    _ => ()
                }
            };
        }
        sum
    }

    fn value(chars: &mut Lexer) -> Value {
        match chars.peek() {
            Some(&'"') => Str(string(chars)),
            Some(&'{') => Int(object(chars)),
            Some(&'[') => Int(array(chars)),
            Some(&c) if c == '-' || c.is_digit(10) => Int(number(chars)),
            _ => unreachable!()
        }
    }

    fn string(chars: &mut Lexer) -> String {
        assert!('"' == chars.next().unwrap());
        let mut value = String::new();
        loop {
            match chars.next() {
                Some('"') => break,
                Some(c) => value.push(c),
                _ => unreachable!(),
            };
        };
        value
    }

    fn number(chars: &mut Lexer) -> i64 {
        let (factor, mut value) = match chars.next() {
            Some('-') => (-1i64, 0u32),
            Some(c) => (1i64, c.to_digit(10).unwrap()),
            _ => unreachable!()
        };
        loop {
            match chars.peek() {
                Some(&c) if c.is_digit(10) => value = value * 10 + c.to_digit(10).unwrap(),
                _ => break,
            };
            chars.next();
        }
        factor * value as i64
    }

    let answer = match chars.peek() {
        Some(&'{') => object(&mut chars),
        Some(&'[') => array(&mut chars),
        _ => unreachable!()
    };

    println!("{}", answer);
}
