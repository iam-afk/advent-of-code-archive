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

fn first_star(_input: &str) -> impl fmt::Display {
    ""
}

fn second_star(_input: &str) -> impl fmt::Display {
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}

}
