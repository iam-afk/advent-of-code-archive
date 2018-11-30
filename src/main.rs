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
    iterate(5, input)
}

fn second_star(input: &str) -> impl fmt::Display {
    input.to_string()
}

fn iterate(n: usize, input: &str) -> usize {
    let rules = read_rules(input.lines());
    let mut grid: Vec<Vec<u8>> = vec![".#.".into(), "..#".into(), "###".into()];
    for _ in 0..n {
        grid = if grid.len() % 2 == 0 {
            iterate2(grid, &rules.rules_2to3)
        } else {
            iterate3(grid, &rules.rules_3to4)
        };
    }
    count_pixels(grid)
}

type Rule2to3 = ([u8; 5], [u8; 11]);
type Rule3to4 = ([u8; 11], [u8; 19]);

#[derive(Debug)]
struct Rules {
    rules_2to3: Vec<Rule2to3>,
    rules_3to4: Vec<Rule3to4>,
}

fn read_rules<'a, T>(lines: T) -> Rules
where
    T: Iterator<Item = &'a str>,
{
    let mut rules_2to3 = vec![];
    let mut rules_3to4 = vec![];
    for line in lines {
        let v: Vec<&str> = line.splitn(2, " => ").collect();
        let from_str = v.first().unwrap();
        let to_str = v.last().unwrap();
        if from_str.len() == 5 {
            let mut from = [0u8; 5];
            from.copy_from_slice(from_str.as_bytes());
            let mut to = [0u8; 11];
            to.copy_from_slice(to_str.as_bytes());
            rules_2to3.push((from, to));
        } else {
            let mut from = [0u8; 11];
            from.copy_from_slice(from_str.as_bytes());
            let mut to = [0u8; 19];
            to.copy_from_slice(to_str.as_bytes());
            rules_3to4.push((from, to));
        }
    }
    Rules {
        rules_2to3,
        rules_3to4,
    }
}

const T2X2: [[usize; 5]; 7] = [
    [3, 0, 2, 4, 1],
    [0, 3, 2, 1, 4],
    [4, 1, 2, 3, 0],
    [1, 4, 2, 0, 3],
    [4, 3, 2, 1, 0],
    [1, 0, 2, 4, 3],
    [3, 4, 2, 0, 1],
];

fn transform_2x2(pattern: &[u8], indexes: &[usize; 5]) -> [u8; 5] {
    let mut result = [0u8; 5];
    for i in 0..5 {
        result[i] = pattern[indexes[i]];
    }
    return result;
}

fn iterate2(grid: Vec<Vec<u8>>, rules: &[Rule2to3]) -> Vec<Vec<u8>> {
    let n = grid.len() / 2;
    let mut new_grid = vec![];
    for _ in 0..3 * n {
        new_grid.push("   ".repeat(n).into_bytes());
    }
    for i in 0..n {
        for j in 0..n {
            let mut patterns = HashSet::new();
            let mut pattern = ['/' as u8; 5];
            &pattern[0..2].copy_from_slice(&grid[2 * i + 0][2 * j..2 * (j + 1)]);
            &pattern[3..5].copy_from_slice(&grid[2 * i + 1][2 * j..2 * (j + 1)]);
            patterns.insert(pattern.clone());

            for indexes in &T2X2 {
                patterns.insert(transform_2x2(&pattern, indexes));
            }

            for (left, right) in rules {
                if patterns.contains(left) {
                    new_grid[3 * i + 0][3 * j..3 * (j + 1)].copy_from_slice(&right[..3]);
                    new_grid[3 * i + 1][3 * j..3 * (j + 1)].copy_from_slice(&right[4..7]);
                    new_grid[3 * i + 2][3 * j..3 * (j + 1)].copy_from_slice(&right[8..]);
                    break;
                }
            }
        }
    }
    new_grid
}

const T3X3: [[usize; 11]; 7] = [
    [8, 4, 0, 3, 9, 5, 1, 7, 10, 6, 2],
    [0, 4, 8, 3, 1, 5, 9, 7, 2, 6, 10],
    [10, 6, 2, 3, 9, 5, 1, 7, 8, 4, 0],
    [2, 6, 10, 3, 1, 5, 9, 7, 0, 4, 8],
    [10, 9, 8, 3, 6, 5, 4, 7, 2, 1, 0],
    [2, 1, 0, 3, 6, 5, 4, 7, 10, 9, 8],
    [8, 9, 10, 3, 4, 5, 6, 7, 0, 1, 2],
];

fn iterate3(grid: Vec<Vec<u8>>, rules: &[Rule3to4]) -> Vec<Vec<u8>> {
    let n = grid.len() / 3;
    let mut new_grid = vec![];
    for _ in 0..4 * n {
        new_grid.push("    ".repeat(n).into_bytes());
    }
    for i in 0..n {
        for j in 0..n {
            let mut patterns = HashSet::new();
            let mut pattern = ['/' as u8; 11];
            &pattern[0..3].copy_from_slice(&grid[3 * i + 0][3 * j..3 * (j + 1)]);
            &pattern[4..7].copy_from_slice(&grid[3 * i + 1][3 * j..3 * (j + 1)]);
            &pattern[8..11].copy_from_slice(&grid[3 * i + 2][3 * j..3 * (j + 1)]);
            patterns.insert(pattern.clone());

            for indexes in &T3X3 {
                patterns.insert(transform_3x3(&pattern, indexes));
            }

            for (left, right) in rules {
                if patterns.contains(left) {
                    new_grid[4 * i + 0][4 * j..4 * (j + 1)].copy_from_slice(&right[..4]);
                    new_grid[4 * i + 1][4 * j..4 * (j + 1)].copy_from_slice(&right[5..9]);
                    new_grid[4 * i + 2][4 * j..4 * (j + 1)].copy_from_slice(&right[10..14]);
                    new_grid[4 * i + 3][4 * j..4 * (j + 1)].copy_from_slice(&right[15..]);
                    break;
                }
            }
        }
    }
    new_grid
}

fn transform_3x3(pattern: &[u8], indexes: &[usize; 11]) -> [u8; 11] {
    let mut result = [0u8; 11];
    for i in 0..11 {
        result[i] = pattern[indexes[i]];
    }
    return result;
}

fn count_pixels(grid: Vec<Vec<u8>>) -> usize {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&v| v == '#' as u8)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let rules = read_rules(
            vec!["../.# => ##./#../...", ".#./..#/### => #..#/..../..../#..#"].into_iter(),
        );
        let grid0: Vec<Vec<u8>> = vec![".#.".into(), "..#".into(), "###".into()];
        let grid1: Vec<Vec<u8>> = vec!["#..#".into(), "....".into(), "....".into(), "#..#".into()];
        let grid2: Vec<Vec<u8>> = vec![
            "##.##.".into(),
            "#..#..".into(),
            "......".into(),
            "##.##.".into(),
            "#..#..".into(),
            "......".into(),
        ];

        assert_eq!(grid1, iterate3(grid0, &rules.rules_3to4));
        assert_eq!(grid2, iterate2(grid1, &rules.rules_2to3));

        assert_eq!(12, count_pixels(grid2));
    }

}
