use std::cmp::max;
use std::collections::HashMap;
use std::io;

const N: usize = 8;

fn main() {
    let mut cities: Vec<String> = vec![];
    let mut m = HashMap::new();
    let mut g = [[0; N]; N];

    let mut line = String::new();
    while io::stdin().read_line(&mut line).unwrap() > 0 {
        {
            let t: Vec<&str> = line.trim().split_whitespace().collect();
            if !m.contains_key(t[0]) {
                m.insert(t[0].to_string(), cities.len());
                cities.push(t[0].to_string());
            }
            if !m.contains_key(t[2]) {
                m.insert(t[2].to_string(), cities.len());
                cities.push(t[2].to_string());
            }
            let u = *m.get(t[0]).unwrap();
            let v = *m.get(t[2]).unwrap();
            let d = t[4].parse::<usize>().unwrap();
            g[u][v] = d;
            g[v][u] = d;
        }
        line.clear();
    }

    fn find(v: usize, n: usize, d: usize, g: &[[usize; N]; N], was: &mut [bool]) -> usize {
        if n == N {
            return d;
        }
        let mut answer = 0;
        was[v] = true;
        for i in 0..N {
            if !was[i] {
                was[i] = true;
                answer = max(answer, find(i, n + 1, d + g[v][i], g, was));
                was[i] = false;
            }
        }
        answer
    }

    let mut answer = 0;
    for s in 0..N {
        let mut was = [false; N];
        answer = max(answer, find(s, 1, 0, &g, &mut was));
    }

    println!("{:?}", answer);
}
