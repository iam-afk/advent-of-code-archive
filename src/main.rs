use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::num;
use std::str;

fn main() -> Result<(), io::Error> {
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input)?;

    println!(" *: {}", first_star(&input));
    println!("**: {}", second_star(&input));

    Ok(())
}

fn first_star(input: &str) -> impl fmt::Display {
    let mut lines: Vec<_> = input.lines().collect();
    let guards = parse_guards(&mut lines);
    strategy_1(&guards)
}

fn second_star(_input: &str) -> impl fmt::Display {
    ""
}

enum Event {
    Guard(usize),
    Fall(usize),
    Wake(usize),
}

impl str::FromStr for Event {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.splitn(7, |c| c == ':' || c == ']' || c == ' ').collect();
        match v[4] {
            "Guard" => Ok(Event::Guard(v[5][1..].parse()?)),
            "falls" => Ok(Event::Fall(v[2].parse()?)),
            "wakes" => Ok(Event::Wake(v[2].parse()?)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Guard {
    id: usize,
    fall: usize,
    wake: usize,
}

fn parse_guards(lines: &mut [&str]) -> Vec<Guard> {
    lines.sort();
    let mut result = vec![];
    let mut current_guard_id = 0usize;
    let mut asleep = 0usize;
    for e in lines.iter().map(|line| line.parse().unwrap()) {
        match e {
            Event::Guard(id) => current_guard_id = id,
            Event::Fall(m) => asleep = m,
            Event::Wake(m) => result.push(Guard {
                id: current_guard_id,
                fall: asleep,
                wake: m,
            }),
        }
    }
    result
}

fn strategy_1(guards: &[Guard]) -> usize {
    let mut guard_minutes = HashMap::new();
    for guard in guards {
        let mut minutes = guard_minutes.entry(guard.id).or_insert(vec![0usize; 60]);
        for c in minutes[guard.fall..guard.wake].iter_mut() {
            *c += 1;
        }
    }
    let id = guard_minutes
        .iter()
        .max_by_key(|&(_, minutes)| minutes.into_iter().filter(|&&v| v > 0).sum::<usize>())
        .map(|(id, _)| id)
        .unwrap();
    let minutes = guard_minutes.get(id).unwrap();
    let minute = minutes
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .map(|(i, _)| i)
        .unwrap();

    id * minute
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let guards = parse_guards(&mut vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ]);
        assert_eq!(10 * 24, strategy_1(&guards));
    }

}
