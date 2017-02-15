use std::env;
use std::io::prelude::*;
use std::io::BufReader;

extern crate hyper;

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::header::Cookie;

fn fetch_input() -> BufReader<Response> {
    let mut cookie = String::from("session=");
    cookie.push_str(&env::args()
        .nth(1)
        .expect("Specify session token as first argument"));
    let client = Client::new();
    let response = client.get("http://adventofcode.com/2016/day/9/input")
        .header(Cookie(vec![cookie]))
        .send()
        .unwrap();
    BufReader::new(response)
}

fn main() {
    let mut file = String::new();
    fetch_input().read_to_string(&mut file).unwrap();
    println!("{}", decompressed_length(&file));
}

fn decompressed_length(file: &str) -> usize {
    let mut chars = file.chars();
    let mut count = 0usize;
    loop {
        match chars.next() {
            None => break,
            Some(x) if x.is_whitespace() => (),
            Some('(') => {
                let mut subsequent_count = 0usize;
                while let Some(d) = chars.next() {
                    if d == 'x' {
                        break;
                    }
                    subsequent_count = subsequent_count * 10 + (d as usize - '0' as usize);
                }
                let mut times = 0usize;
                while let Some(d) = chars.next() {
                    if d == ')' {
                        break;
                    }
                    times = times * 10 + (d as usize - '0' as usize);
                }
                count += subsequent_count * times;
                for _ in 0..subsequent_count {
                    chars.next();
                }

            }
            _ => count += 1,
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(decompressed_length("ADVENT"), 6);
        assert_eq!(decompressed_length("A(1x5)BC"), 7);
        assert_eq!(decompressed_length("(3x3)XYZ"), 9);
        assert_eq!(decompressed_length("A(2x2)BCD(2x2)EFG"), 11);
        assert_eq!(decompressed_length("(6x1)(1x3)A"), 6);
        assert_eq!(decompressed_length("X(8x2)(3x3)ABCY"), 18);
    }

}
