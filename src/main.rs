use std::fmt;
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), io::Error> {
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input)?;

    println!(" *: {}", first_star(&input));
    println!("**: {}", second_star(&input));

    Ok(())
}

fn first_star(input: &str) -> impl fmt::Display {
    react(input.trim().bytes())
}

fn second_star(input: &str) -> impl fmt::Display {
    shortest(input.trim())
}

fn react<C>(polymer: C) -> usize
where
    C: Iterator<Item = u8>,
{
    let mut v = vec![];
    for unit in polymer {
        let &previous = v.last().unwrap_or(&0u8);
        if unit != previous && unit.to_ascii_lowercase() == previous.to_ascii_lowercase() {
            v.pop();
        } else {
            v.push(unit);
        }
    }
    v.len()
}

fn shortest(polymer: &str) -> usize {
    ('a' as u8..='z' as u8)
        .map(|x| react(polymer.bytes().filter(|c| c.to_ascii_lowercase() != x)))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(10, react("dabAcCaCBAcCcaDA".bytes()));
        assert_eq!(4, shortest("dabAcCaCBAcCcaDA"));
    }

}
