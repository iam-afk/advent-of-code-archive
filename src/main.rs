const A: u8 = 'a' as u8;
const I: u8 = 'i' as u8 - A;
const O: u8 = 'o' as u8 - A;
const L: u8 = 'l' as u8 - A;

fn main() {
    let mut pwd: Vec<u8> = "cqjxxyzz".as_bytes()
        .iter()
        .map(|&x| x - A)
        .collect();
    pwd.reverse();
    loop {
        for i in 0..8 {
            if pwd[i] + 1 < 26 {
                pwd[i] += 1;
                break;
            } else {
                pwd[i] = 0;
            }
        }
        let inc_straight = (2..8).any(|x| pwd[x] + 1 == pwd[x - 1] && pwd[x] + 2 == pwd[x - 2]);
        let not_contain = pwd.iter().all(|&x| x != I && x != O && x != L);
        let two_pairs = {
            let pair_of_letters = |x| pwd[x] == pwd[x + 1];
            if let Some(k) = (0..7).position(&pair_of_letters) {
                (k+2..7).any(&pair_of_letters)
            } else {
                false
            }
        };

        if inc_straight && not_contain && two_pairs {
            pwd.reverse();
            let answer: String = pwd.iter()
                .map(|&x| (x + A) as char)
                .collect();
            println!("{}", answer);
            break;
        }
    }
}
