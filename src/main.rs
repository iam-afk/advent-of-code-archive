use std::io;

const FINISH: usize = 2503;

enum State { FLYING, RESTING }

use State::*;

struct Reindeer {
    speed: usize,
    time: usize,
    rest_time: usize,
    state: State,
    rest: usize,
    position: usize,
    points: usize
}

fn main() {
    let mut reindeers = vec![];
    let stdin = io::stdin();
    let mut line = String::new();
    while stdin.read_line(&mut line).unwrap() > 0 {
        {
            let w: Vec<_> = line.split_whitespace().collect();
            let time = w[6].parse::<usize>().unwrap();
            reindeers.push(Reindeer {
                speed: w[3].parse::<usize>().unwrap(),
                time: time,
                rest_time: w[13].parse::<usize>().unwrap(),
                state: FLYING,
                rest: time,
                position: 0,
                points: 0
            });
        }
        line.clear();
    }
    for _ in 0..FINISH {
        for r in reindeers.iter_mut() {
            if r.rest == 0 {
                match r.state {
                    FLYING => { r.state = RESTING; r.rest = r.rest_time; },
                    RESTING => { r.state = FLYING; r.rest = r.time; }
                };
            }
            match r.state {
                 FLYING => r.position += r.speed,
                 _ => ()
            }
            r.rest -= 1;
        }
        reindeers.sort_by(|x, y| y.position.cmp(&x.position));
        let leader_position = reindeers[0].position;
        for r in reindeers.iter_mut().take_while(|r| r.position == leader_position) {
            r.points += 1;
        }
    }
    let answer = reindeers.iter().map(|r| r.points).max().unwrap();
    println!("{}", answer);
}
