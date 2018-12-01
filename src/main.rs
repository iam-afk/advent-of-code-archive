use std::collections::HashSet;
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
    let result: i32 = input.lines().map(|line| line.parse::<i32>().unwrap()).sum();
    result
}

fn second_star(input: &str) -> impl fmt::Display {
    let changes: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut reached = HashSet::new();
    let mut current = 0i32;
    for change in changes.iter().cycle() {
        if !reached.insert(current) {
            return current;
        }
        current += change
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}

}
