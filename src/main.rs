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

fn second_star(input: &str) -> impl fmt::Display {
    let v: Vec<_> = input.lines().collect();
    common_correct_id(&v)
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

fn common_correct_id(ids: &[&str]) -> String {
    for id1 in ids {
        for id2 in ids {
            let common_id: String = id1
                .chars()
                .zip(id2.chars())
                .filter(|(c1, c2)| c1 == c2)
                .map(|(_, c)| c)
                .collect();
            if common_id.len() == id1.len() - 1 {
                return common_id;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_1() {
        assert_eq!(
            12,
            checksum(&["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",])
        );
    }

    #[test]
    fn examples_2() {
        assert_eq!(
            "fgij",
            common_correct_id(&["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",])
        );
    }

}
