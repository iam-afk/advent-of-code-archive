extern crate regex;

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::prelude::*;

use regex::Regex;

fn main() -> Result<(), io::Error> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;

    let input = buffer.trim_end();
    println!(" *: {}", first_star(&input));
    println!("**: {}", second_star(&input));

    Ok(())
}

fn parse(input: &str) -> ([i32; 4], [i32; 4], [i32; 4]) {
    let mut xs = [0; 4];
    let mut ys = [0; 4];
    let mut zs = [0; 4];

    let numbers = Regex::new(r"-?\d+").unwrap();
    for (i, line) in input.lines().enumerate() {
        match numbers
            .find_iter(line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .as_slice()
        {
            &[x, y, z] => {
                xs[i] = x;
                ys[i] = y;
                zs[i] = z;
            }
            _ => (),
        }
    }
    (xs, ys, zs)
}

fn delta(a: i32, b: i32) -> i32 {
    a.cmp(&b) as i32
}

fn first_star(input: &str) -> impl fmt::Display {
    let (mut x, mut y, mut z) = parse(input);
    let mut vx = [0; 4];
    let mut vy = [0; 4];
    let mut vz = [0; 4];
    for _ in 0..1000 {
        for i in 0..3 {
            for j in (i + 1)..4 {
                let d = delta(x[i], x[j]);
                vx[i] += -d;
                vx[j] += d;
                let d = delta(y[i], y[j]);
                vy[i] += -d;
                vy[j] += d;
                let d = delta(z[i], z[j]);
                vz[i] += -d;
                vz[j] += d;
            }
        }
        for i in 0..4 {
            x[i] += vx[i];
            y[i] += vy[i];
            z[i] += vz[i];
        }
    }
    (0..4)
        .map(|i| (x[i].abs() + y[i].abs() + z[i].abs()) * (vx[i].abs() + vy[i].abs() + vz[i].abs()))
        .sum::<i32>()
}

fn second_star(input: &str) -> impl fmt::Display {
    let (x, y, z) = parse(input);

    fn period(a: [i32; 4]) -> usize {
        let mut b = a.clone();
        let mut v = [0; 4];
        let mut step = 0usize;
        let mut pos = HashMap::new();
        loop {
            step += 1;
            for i in 0..3 {
                for j in (i + 1)..4 {
                    let d = delta(b[i], b[j]);
                    v[i] += -d;
                    v[j] += d;
                }
            }
            for i in 0..4 {
                b[i] += v[i];
            }
            if v.iter().all(|x| *x == 0) {
                if let Some(prev) = pos.get(&b) {
                    return step - prev;
                }
                pos.insert(b.clone(), step);
            }
        }
    }
    let sx = period(x);
    let sy = period(y);
    let sz = period(z);

    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        return a;
    }

    let sxy = sx / gcd(sx, sy) * sy;
    sxy / gcd(sxy, sz) * sz
}
