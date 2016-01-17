use std::cmp;
use std::io;

const FINISH: usize = 2503;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    let mut answer = 0usize;
    while stdin.read_line(&mut line).unwrap() > 0 {
        {
            let w: Vec<_> = line.split_whitespace().collect();
            let speed = w[3].parse::<usize>().unwrap();
            let time = w[6].parse::<usize>().unwrap();
            let rest_time = w[13].parse::<usize>().unwrap();

            let t = FINISH % (time + rest_time);
            let d = speed * (FINISH / (time + rest_time) * time + cmp::min(t, time));
            answer = cmp::max(answer, d);
        }
        line.clear();
    }
    println!("{}", answer);
}
