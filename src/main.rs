use std::io;

fn main() {
    let stdin = io::stdin();
    let mut answer = 0usize;
    let mut line = String::new();
    while stdin.read_line(&mut line).unwrap() > 0 {
        {
            let s = line.trim();
            let mut chars = s.chars();
            while let Some(c) = chars.next() {
                answer += match c {
                    '"' | '\\' => 2,
                    _ => 1
                }
            }
            answer += 2;
            answer -= s.len();
        }
        line.clear();
    }
    println!("{}", answer);
}
