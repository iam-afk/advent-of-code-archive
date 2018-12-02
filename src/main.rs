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
    let v: Vec<_> = input.lines().collect();
    checksum(&v)
}

fn second_star(_input: &str) -> impl fmt::Display {
    ""
}

fn checksum(ids: &[&str]) -> usize {
    let mut two = 0usize;
    let mut three = 0usize;

    for id in ids {
        let mut letters = [0usize; 26];
        for c in id.chars() {
            letters[c as usize - 'a' as usize] += 1;
        }
        let mut exactly_two = false;
        let mut exactly_three = false;
        for &count in &letters {
            if count == 2 {
                exactly_two = true
            }
            if count == 3 {
                exactly_three = true
            }
        }
        if exactly_two {
            two += 1
        }
        if exactly_three {
            three += 1
        }
    }
    two * three
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            12,
            checksum(&["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",])
        );
    }

}
