use std::collections::HashMap;
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
    let lines: Vec<_> = input.lines().collect();
    find_order(read_instructions(&lines))
}

fn second_star(input: &str) -> impl fmt::Display {
    let lines: Vec<_> = input.lines().collect();
    find_time(read_instructions(&lines), 60, 5)
}

#[derive(Debug)]
struct Instructions {
    steps: [bool; 26],
    required_for: HashMap<usize, Vec<usize>>,
    amount: [u32; 26],
}

fn read_instructions(lines: &[&str]) -> Instructions {
    let mut steps = [false; 26];
    let mut required_for = HashMap::new();
    let mut amount = [0u32; 26];
    for line in lines {
        let mut b = line.bytes();
        let v = b.nth(5).unwrap() as usize - 'A' as usize;
        steps[v] = true;
        let u = b.nth(30).unwrap() as usize - 'A' as usize;
        steps[u] = true;
        required_for
            .entry(v)
            .or_insert_with(|| Vec::with_capacity(4))
            .push(u);
        amount[u as usize] += 1;
    }
    Instructions {
        steps,
        required_for,
        amount,
    }
}

fn find_order(mut instructions: Instructions) -> String {
    let mut order = String::new();
    let n = instructions.steps.iter().filter(|&&s| s).count();
    for _ in 0..n {
        let s = (0..n)
            .filter(|&s| instructions.steps[s])
            .min_by_key(|&s| instructions.amount[s])
            .unwrap();
        order.push(('A' as u8 + s as u8) as char);
        instructions.steps[s] = false;
        match instructions.required_for.get(&s) {
            Some(v) => {
                for &t in v {
                    instructions.amount[t] -= 1;
                }
            }
            _ => (),
        }
    }

    order
}

fn find_time(mut instructions: Instructions, duration: usize, worker_count: usize) -> usize {
    let n = instructions.steps.iter().filter(|&&s| s).count();
    let mut workers = vec![(0usize, n); worker_count];
    let mut completed = 0;
    loop {
        workers.sort();
        let &(t, _) = workers.first().unwrap();
        for &(ct, s) in workers.iter() {
            if s < n && ct == t {
                match instructions.required_for.get(&s) {
                    Some(v) => {
                        for &t in v {
                            instructions.amount[t] -= 1;
                        }
                    }
                    _ => (),
                }
                completed += 1;
                if completed == n {
                    return workers.iter().map(|&(t, _)| t).max().unwrap();
                }
            }
        }
        workers[0] = match (0..n)
            .filter(|&s| instructions.steps[s] && instructions.amount[s] == 0)
            .min()
        {
            Some(s) => {
                instructions.steps[s] = false;
                (t + s + duration + 1, s)
            }
            _ => (t + 1, n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let instructions = read_instructions(&[
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ]);
        assert_eq!("CABDFE", find_order(instructions));
    }

    #[test]
    fn example_2() {
        let instructions = read_instructions(&[
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ]);
        assert_eq!(15, find_time(instructions, 0, 2));
    }

}
