use std::env;
use std::fmt;
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
    let response = client.get("http://adventofcode.com/2016/day/8/input")
        .header(Cookie(vec![cookie]))
        .send()
        .unwrap();
    BufReader::new(response)
}

fn main() {
    let mut screen = Screen::new(6, 50);
    for line in fetch_input().lines() {
        let op = line.unwrap();
        screen.do_operation(&op);
    }
    println!("{}\n{:?}", screen.count(), screen);
}

struct Screen {
    rows: usize,
    columns: usize,
    pixels: Box<[u8]>,
}

impl Screen {
    
    fn new(rows: usize, columns: usize) -> Screen {
        Screen {
            rows: rows,
            columns: columns,
            pixels: vec![0u8; rows * columns].into_boxed_slice(),
        }
    }
    
    fn do_operation(&mut self, op: &str) {
        let token: Vec<&str> = op.split_whitespace().collect();
        match token[0] {
            "rect" => {
                let x = token[1].find('x').unwrap();
                let wide = token[1][0..x].parse::<usize>().unwrap();
                let tall = token[1][x + 1..].parse::<usize>().unwrap();
                for i in 0..tall {
                    for j in 0..wide {
                        self.pixels[i * self.columns + j] = 1;
                    }
                }
            }
            "rotate" => {
                match token[1] {
                    "row" => {
                        let row = token[2][2..].parse::<usize>().unwrap();
                        let by = token[4].parse::<usize>().unwrap();
                        assert!(by < self.columns);
                        let start = row * self.columns;
                        let end = (row + 1) * self.columns;
                        self.pixels[start..end].reverse();
                        self.pixels[start..start+by].reverse();
                        self.pixels[start+by..end].reverse();
                    }
                    "column" => {
                        let column = token[2][2..].parse::<usize>().unwrap();
                        let by = token[4].parse::<usize>().unwrap();
                        assert!(by < self.rows);
                        let end = self.rows;
                        self.reverse_column(column, 0, end);
                        self.reverse_column(column, 0, by);
                        self.reverse_column(column, by, end);
                    }
                    _ => unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
    
    fn reverse_column(&mut self, column: usize, start: usize, end: usize) {
        let mut i: usize = 0;
        let ln = end - start;
        while i < ln / 2 {
            self.pixels.swap(
                (start + i) * self.columns + column,
                (start + ln - i - 1) * self.columns + column);
            i += 1;
        }
    }

    fn count(&self) -> u8 {
        self.pixels.iter().sum()
    }
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.columns {
                match self.pixels[i * self.columns + j] {
                    0 => write!(f, " ").unwrap(),
                    1 => write!(f, "#").unwrap(),
                    _ => unreachable!()
                }
            }
            writeln!(f, "").unwrap()
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn examples() {
        let mut screen = Screen::new(3, 7);
        assert_eq!(*screen.pixels, [
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ]);

        screen.do_operation("rect 3x2");
        assert_eq!(*screen.pixels, [
            1, 1, 1, 0, 0, 0, 0,
            1, 1, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ]);

        screen.do_operation("rotate column x=1 by 1");
        assert_eq!(*screen.pixels, [
            1, 0, 1, 0, 0, 0, 0,
            1, 1, 1, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 0,
        ]);

        screen.do_operation("rotate row y=0 by 4");
        assert_eq!(*screen.pixels, [
            0, 0, 0, 0, 1, 0, 1,
            1, 1, 1, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 0,
        ]);

        screen.do_operation("rotate column x=1 by 1");
        assert_eq!(*screen.pixels, [
            0, 1, 0, 0, 1, 0, 1,
            1, 0, 1, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 0, 0,
        ]);
        
    }

}
