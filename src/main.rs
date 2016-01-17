use std::io;
use std::collections::HashMap;
use std::cmp;

static IGNORED: &'static [char] = &['.', '\n'];

fn main() {
    let mut g = [[0i32; 8]; 8];
    let mut names = HashMap::new();

    {
        let mut index = |name: &str| {
            let next = names.len();
            *names.entry(name.to_string()).or_insert(next)
        };

        let stdin = io::stdin();
        let mut line = String::new();
        while stdin.read_line(&mut line).unwrap() > 0 {
            {
                let w: Vec<_> = line.trim_matches(IGNORED).split_whitespace().collect();
                let factor = match w[2] {
                    "gain" => 1i32,
                    "lose" => -1i32,
                    _ => unreachable!()
                };
                let happiness = w[3].parse::<i32>().unwrap();
                g[index(w[0])][index(w[10])] = factor * happiness;
            }
            line.clear();
        }
    }

    fn sit_from(k: usize, n: usize, u: &mut [usize; 8], g: &[[i32; 8]; 8]) -> i32 {
        if k < n {
            let mut result = i32::min_value();
            for i in 0..n {
                if u[i] == usize::max_value() {
                    u[i] = k;
                    result = cmp::max(result, sit_from(k + 1, n, u, g));
                    u[i] = usize::max_value();
                }
            }
            result
        } else {
            let mut happiness = g[u[0]][u[n - 1]] + g[u[n - 1]][u[0]];
            for i in 0..n - 1 {
                happiness += g[u[i]][u[i + 1]];
                happiness += g[u[i + 1]][u[i]];
            }
            happiness
        }
    }
    let mut u = [usize::max_value(); 8];
    println!("{}", sit_from(0, names.len(), &mut u, &g));
}
