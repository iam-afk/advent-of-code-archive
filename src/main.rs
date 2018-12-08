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

fn second_star(input: &str) -> impl fmt::Display {
    let mut tree = nodes(input);
    value_of_root(&mut tree)
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

fn value_of_root<T>(tree: &mut T) -> usize
where
    T: Iterator<Item = usize>,
{
    let childs = tree.next().unwrap();
    let entries = tree.next().unwrap();
    if childs == 0 {
        return tree.take(entries).sum();
    }
    let value_of_child: Vec<_> = (0..childs).map(|_| value_of_root(tree)).collect();
    tree.take(entries)
        .filter(|&i| 0 < i && i <= childs)
        .map(|i| value_of_child[i - 1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut tree = nodes("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(138, sum_metadata(&mut tree));
    }

    #[test]
    fn example_2() {
        let mut tree = nodes("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(66, value_of_root(&mut tree));
    }

}
