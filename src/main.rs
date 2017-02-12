use std::collections::HashSet;

use std::env;
use std::io::prelude::*;
use std::io::BufReader;

extern crate hyper;

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::header::Cookie;

fn is_ssl_supported(ip: &str) -> bool {
    let mut a = '\0';
    let mut b = '\0';
    let mut in_hypernet = false;
    let mut aba = HashSet::new();
    let mut bab = HashSet::new();
    for c in ip.chars() {
        match c {
            '[' => in_hypernet = true,
            ']' => in_hypernet = false,
            _ => {
                if a == c && a != b && b != '[' && b != ']' {
                    if in_hypernet {
                        bab.insert((b, a));
                    } else {
                        aba.insert((a, b));
                    }
                }
            }
        }
        a = b;
        b = c;
    }
    aba.intersection(&bab).count() > 0
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
                 .filter(|s| is_ssl_supported(s))
                 .count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(is_ssl_supported("aba[bab]xyz"));
        assert!(!is_ssl_supported("xyx[xyx]xyx"));
        assert!(is_ssl_supported("aaa[kek]eke"));
        assert!(is_ssl_supported("zazbz[bzb]cdb"));
    }

}
