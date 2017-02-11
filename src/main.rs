use std::env;
use std::io::prelude::*;
use std::io::BufReader;

extern crate hyper;

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::header::Cookie;

fn is_tls_supported(ip: &str) -> bool {
    let mut x = '\0';
    let mut y = '\0';
    let mut z = '\0';
    let mut supported = false;
    let mut hypernet = false;
    for c in ip.chars() {
        if c == '[' {
            hypernet = true;
        }
        if c == ']' {
            hypernet = false;
        }
        if x == c && y == z && x != y {
            if hypernet {
                return false;
            }
            supported = true;
        }
        x = y;
        y = z;
        z = c;
    }
    supported
}

fn fetch_input() -> BufReader<Response> {
    let mut cookie = String::from("session=");
    cookie.push_str(&env::args().nth(1).unwrap());
    let client = Client::new();
    let response = client.get("http://adventofcode.com/2016/day/7/input")
        .header(Cookie(vec![cookie]))
        .send()
        .unwrap();
    BufReader::new(response)
}

fn main() {
    println!("{}",
             fetch_input()
                 .lines()
                 .map(|s| s.unwrap())
                 .filter(|s| is_tls_supported(&s))
                 .count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(is_tls_supported("abba[mnop]qrst"));
        assert!(!is_tls_supported("abcd[bddb]xyyx"));
        assert!(!is_tls_supported("aaaa[qwer]tyui"));
        assert!(is_tls_supported("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn abba_within_hypernet_sequence() {
        assert!(!is_tls_supported("abba[xyyx]abba"));
    }

}
