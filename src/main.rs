use std::fmt;
use std::io;
use std::io::prelude::*;

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
    react(input.bytes())
}

fn second_star(input: &str) -> impl fmt::Display {
    shortest(input)
}

fn react<C>(polymer: C) -> usize
where
    C: Iterator<Item = u8>,
{
    let mut v = vec![];
    for unit in polymer {
        let &previous = v.last().unwrap_or(&0u8);
        if unit ^ 0x20 == previous {
            v.pop();
        } else {
            v.push(unit);
        }
    }
    v.len()
}

fn shortest(polymer: &str) -> usize {
    ('a' as u8..='z' as u8)
        .map(|x| react(polymer.bytes().filter(|c| c | 0x20 != x)))
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
