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
    react(input.trim())
}

fn second_star(_input: &str) -> impl fmt::Display {
    ""
}

fn react(polymer: &str) -> usize {
    let mut v = vec![];
    for unit in polymer.chars() {
        let &previous = v.last().unwrap_or(&'.');
        if unit != previous && unit.to_ascii_lowercase() == previous.to_ascii_lowercase() {
            v.pop();
        } else {
            v.push(unit);
        }
    }
    v.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(10, react("dabAcCaCBAcCcaDA"));
    }

}
