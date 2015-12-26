use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut origin = line.trim().to_string();
    let mut answer = 0;
    for _ in 0..40 {
        let mut result = String::new();
        {
            let mut chars = origin.chars();
            let mut first = chars.next().unwrap();
            let mut count = 1;
            while let Some(ch) = chars.next() {
                if first == ch {
                    count += 1;
                } else {
                    result.push((48u8 + count) as char);
                    result.push(first);
                    first = ch;
                    count = 1;
                }
            }
            result.push((48u8 + count) as char);
            result.push(first);
        }
        answer = result.len();
        origin = result;
    }
    println!("{}", answer);
}
