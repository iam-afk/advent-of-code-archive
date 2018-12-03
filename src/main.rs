use std::fmt;
use std::io;
use std::io::prelude::*;
use std::num;
use std::str;

fn main() -> Result<(), io::Error> {
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input)?;

    println!(" *: {}", first_star(&input));
    println!("**: {}", second_star(&input));

    Ok(())
}

fn first_star(input: &str) -> impl fmt::Display {
    let rectangles = read_rectangles(input.lines());
    count_within_two_or_more(&rectangles)
}

fn second_star(input: &str) -> impl fmt::Display {
    let rectangles = read_rectangles(input.lines());
    find_id_of_not_overlapped_claim(&rectangles)
}

struct Rectangle {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl str::FromStr for Rectangle {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s
            .splitn(7, |c| c == ' ' || c == ',' || c == ':' || c == 'x')
            .collect();
        Ok(Rectangle {
            id: v[0][1..].parse()?,
            left: v[2].parse()?,
            top: v[3].parse()?,
            width: v[5].parse()?,
            height: v[6].parse()?,
        })
    }
}

fn read_rectangles<'a, I>(input: I) -> Vec<Rectangle>
where
    I: Iterator<Item = &'a str>,
{
    input
        .map(|line| line.parse::<Rectangle>().unwrap())
        .collect()
}

fn count_within_two_or_more(rectangles: &[Rectangle]) -> usize {
    let mut fabric = vec![vec![0usize; 1000]; 1000];
    for r in rectangles {
        for i in 0..r.height {
            for j in 0..r.width {
                fabric[r.top + i][r.left + j] += 1;
            }
        }
    }
    fabric
        .into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|&v| v > 1)
        .count()
}

fn find_id_of_not_overlapped_claim(rectangles: &[Rectangle]) -> usize {
    let mut fabric = vec![vec![0usize; 1000]; 1000];
    let mut overlapped = vec![true];
    for r in rectangles {
        let mut current = false;
        for i in 0..r.height {
            for j in 0..r.width {
                let id = fabric[r.top + i][r.left + j];
                if id > 0 {
                    overlapped[id] = true;
                    current = true;
                }
                fabric[r.top + i][r.left + j] = r.id;
            }
        }
        overlapped.push(current);
    }
    overlapped
        .iter()
        .enumerate()
        .find(|&(_, v)| !v)
        .map(|(i, _)| i)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let rectangles =
            read_rectangles(vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"].into_iter());
        assert_eq!(4, count_within_two_or_more(&rectangles));
        assert_eq!(3, find_id_of_not_overlapped_claim(&rectangles));
    }

}
