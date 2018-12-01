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

fn second_star(_input: &str) -> impl fmt::Display {
    ""
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let p0: Particle = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>".parse().unwrap();
        let p1: Particle = "p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>".parse().unwrap();

        assert_eq!(0, closest(&[p0, p1]));
    }

}
