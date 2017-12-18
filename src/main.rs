use std::error::Error;
use std::io::{Read, stdin};

fn with_day_input<F1, T1, F2, T2>(f1: F1, f2: F2) -> Result<(), Box<Error>>
where
    F1: Fn(&str) -> T1,
    F2: Fn(&str) -> T2,
    T1: std::fmt::Display,
    T2: std::fmt::Display,
{
    let mut stdin = stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input)?;
    println!(" *: {}", f1(&input));
    println!("**: {}", f2(&input));
    Ok(())
}

fn main() {
    with_day_input(|input| input.to_string(), |input| input.to_string()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}

}
