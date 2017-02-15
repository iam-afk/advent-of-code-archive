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
    let mut file = Vec::new();
    fetch_input().read_to_end(&mut file).unwrap();
    println!("{}", decompressed_length(&file));
}

const ZERO: usize = '0' as usize;
const LPAREN: u8 = '(' as u8;
const X: u8 = 'x' as u8;
const RPAREN: u8 = ')' as u8;

fn decompressed_length(file: &[u8]) -> usize {
    let mut count = 0usize;
    let mut start = 0usize;
    let end = file.len();
    while start < end {
        match file[start] {
            LPAREN => {
                let mut subsequent_count = 0usize;
                let mut k = start + 1;
                while file[k] != X {
                    subsequent_count = subsequent_count * 10 + file[k] as usize - ZERO;
                    k += 1
                }
                let mut times = 0usize;
                k += 1;
                while file[k] != RPAREN {
                    times = times * 10 + file[k] as usize - ZERO;
                    k += 1;
                }
                start = k + 1;
                count += times * decompressed_length(&file[start..start + subsequent_count]);
                start = start + subsequent_count;
            }
            _ => {
                if !(file[start] as char).is_whitespace() {
                    count += 1;
                }
                start += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(decompressed_length("(3x3)XYZ".as_bytes()), 9);
        assert_eq!(decompressed_length("X(8x2)(3x3)ABCY".as_bytes()), 20);
        assert_eq!(decompressed_length("(27x12)(20x12)(13x14)(7x10)(1x12)A".as_bytes()),
                   241920);
        assert_eq!(decompressed_length("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"
                       .as_bytes()),
                   445);
    }

}
