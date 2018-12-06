use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::num;
use std::str;

fn main() -> Result<(), io::Error> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;

    let input = buffer.trim_end();
    println!(" *: {}", first_star(&input));
    println!("**: {}", second_star(&input));

    Ok(())
}

fn first_star(input: &str) -> impl fmt::Display {
    let points: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    largest(&points)
}

fn second_star(_input: &str) -> impl fmt::Display {
    ""
}

#[derive(Hash, PartialEq, Eq)]
struct Point(i32, i32);

impl str::FromStr for Point {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s.splitn(2, ", ").map(|s| s.parse::<i32>().unwrap());
        let row = v.next().unwrap();
        let col = v.next().unwrap();
        Ok(Point(row, col))
    }
}

fn largest(points: &[Point]) -> u32 {
    let mut areas = HashMap::new();
    for point in points.iter() {
        areas.insert(point, 0u32);
    }

    let min_row = points.iter().map(|p| p.0).min().unwrap();
    let max_row = points.iter().map(|p| p.0).max().unwrap();
    let min_col = points.iter().map(|p| p.1).min().unwrap();
    let max_col = points.iter().map(|p| p.1).max().unwrap();

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            let mut v: Vec<_> = points
                .iter()
                .map(|p| (p, (p.0 - r).abs() + (p.1 - c).abs()))
                .collect();
            v.sort_by_key(|&(_, d)| d);
            if v[0].1 < v[1].1 {
                if r == min_row || r == max_row || c == min_col || c == max_col {
                    areas.remove(v[0].0);
                } else {
                    areas.entry(&v[0].0).and_modify(|e| *e += 1);
                }
            }
        }
    }

    *areas.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let points = vec![
            Point(1, 1),
            Point(1, 6),
            Point(8, 3),
            Point(3, 4),
            Point(5, 5),
            Point(8, 9),
        ];
        assert_eq!(17, largest(&points));
    }

}
