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
    let particles: Vec<Particle> = input.lines().map(|line| line.parse().unwrap()).collect();
    closest(&particles)
}

fn second_star(input: &str) -> impl fmt::Display {
    let mut particles: Vec<Particle> = input.lines().map(|line| line.parse().unwrap()).collect();
    count_remaining(&mut particles)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct V3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Particle {
    p: V3,
    v: V3,
    a: V3,
}

impl str::FromStr for Particle {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<_> = s.split(|c| c == '<' || c == ',' || c == '>').collect();
        let p = V3 {
            x: s[1].parse()?,
            y: s[2].parse()?,
            z: s[3].parse()?,
        };
        let v = V3 {
            x: s[6].parse()?,
            y: s[7].parse()?,
            z: s[8].parse()?,
        };
        let a = V3 {
            x: s[11].parse()?,
            y: s[12].parse()?,
            z: s[13].parse()?,
        };
        Ok(Particle { p: p, v: v, a: a })
    }
}

fn closest(particles: &[Particle]) -> usize {
    let (index, _) = particles
        .iter()
        .enumerate()
        .min_by_key(|&(_, p)| p.a.x * p.a.x + p.a.y * p.a.y + p.a.z * p.a.z)
        .unwrap();
    return index;
}

fn count_remaining(particles: &mut [Particle]) -> usize {
    let mut alive = particles.len();
    let mut destroyed = vec![false; alive];
    for _ in 0..1_000 {
        let mut at_position = HashMap::with_capacity(alive);
        for (i, p) in particles
            .iter_mut()
            .enumerate()
            .filter(|&(i, _)| !destroyed[i])
        {
            p.v.x += p.a.x;
            p.v.y += p.a.y;
            p.v.z += p.a.z;
            p.p.x += p.v.x;
            p.p.y += p.v.y;
            p.p.z += p.v.z;
            let at = at_position
                .entry(p.p.clone())
                .or_insert(Vec::with_capacity(4));
            at.push(i);
        }

        for indexes in at_position.values() {
            if indexes.len() > 1 {
                for i in indexes {
                    destroyed[*i] = true;
                    alive -= 1;
                }
            }
        }
    }
    alive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let p0: Particle = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>".parse().unwrap();
        let p1: Particle = "p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>".parse().unwrap();

        assert_eq!(0, closest(&[p0, p1]));
    }

    #[test]
    fn example_2() {
        let mut particles: Vec<Particle> = vec![
            "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>".parse().unwrap(),
            "p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>".parse().unwrap(),
            "p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>".parse().unwrap(),
            "p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>".parse().unwrap(),
        ];
        assert_eq!(1, count_remaining(&mut particles));
    }

}
