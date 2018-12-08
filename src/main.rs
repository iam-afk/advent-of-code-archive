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
    let mut tree = nodes(input);
    sum_metadata(&mut tree)
}

fn second_star(_input: &str) -> impl fmt::Display {
    ""
}

fn nodes<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input.split_whitespace().map(|line| line.parse().unwrap())
}

fn sum_metadata<T>(tree: &mut T) -> usize
where
    T: Iterator<Item = usize>,
{
    let childs = tree.next().unwrap();
    let entries = tree.next().unwrap();
    (0..childs).map(|_| sum_metadata(tree)).sum::<usize>() + tree.take(entries).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut tree = nodes("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(138, sum_metadata(&mut tree));
    }

}
