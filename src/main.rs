use std::io;

fn main() {
    let stdin = io::stdin();
    let mut answer = 0usize;
    let mut line = String::new();
    while stdin.read_line(&mut line).unwrap() > 0 {
        {
            let s = line.trim();
            answer += s.len() + 2;
            let mut chars = s.chars();
            while let Some(c) = chars.next() {
                if c == '\\' {
                    if let Some('x') = chars.next() {
                        chars.next();
                        chars.next();
                    }
                };
                answer -= 1
            }
        }
        line.clear();
    }
    println!("{}", answer);
}
