use std::io;
use std::io::Read;
use std::str::Chars;
use std::iter::Peekable;

type Lexer<'a> = Peekable<Chars<'a>>;

fn main() {
    let mut stdin = io::stdin();
    let mut data = String::new();
    stdin.read_to_string(&mut data).unwrap();
    let mut chars = data.trim().chars().peekable();

    fn object(chars: &mut Lexer) -> i64 {
        assert!('{' == chars.next().unwrap());
        let mut sum = 0i64;
        loop {
            match chars.peek() {
                Some(&c) if c == '}' => { chars.next(); return sum },
                Some(&c) if c == ',' => { chars.next(); },
                _ => {
                    string(chars);
                    assert!(':' == chars.next().unwrap());
                    sum += value(chars);
                }
            };
        }
    }

    fn array(chars: &mut Lexer) -> i64 {
        assert!('[' == chars.next().unwrap());
        let mut sum = 0i64;
        loop {
            match chars.peek() {
                Some(&c) if c == ']' => { chars.next(); return sum },
                Some(&c) if c == ',' => { chars.next(); },
                _ => sum += value(chars)
            };
        }
    }

    fn value(chars: &mut Lexer) -> i64 {
        match chars.peek() {
            Some(&c) if c == '"' => { string(chars); 0i64 },
            Some(&c) if c == '{' => object(chars),
            Some(&c) if c == '[' => array(chars),
            Some(&c) if c == '-' || c.is_digit(10) => number(chars),
            _ => unreachable!()
        }
    }

    fn string(chars: &mut Lexer) {
        assert!('"' == chars.next().unwrap());
        loop {
            match chars.next() {
                None => unreachable!(),
                Some('"') => break,
                _ => ()
            };
        }
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
        Some(&c) if c == '{' => object(&mut chars),
        Some(&c) if c == '[' => array(&mut chars),
        _ => unreachable!()
    };

    println!("{}", answer);
}
