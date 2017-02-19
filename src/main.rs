use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::io::BufReader;

extern crate hyper;

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::header::Cookie;

fn fetch_input() -> BufReader<Response> {
    let mut cookie = String::from("session=");
    cookie.push_str(&env::args()
        .nth(1)
        .expect("Specify session token as first argument"));
    let client = Client::new();
    let response = client.get("http://adventofcode.com/2016/day/10/input")
        .header(Cookie(vec![cookie]))
        .send()
        .unwrap();
    BufReader::new(response)
}

fn main() {
    let instructions: Vec<_> = fetch_input()
        .lines()
        .map(|s| s.unwrap())
        .collect();
    println!("{}", number_of_bot(instructions, 61, 17))
}

#[derive(Debug)]
enum Dest {
    Bot(u32),
    Output(u32),
}

impl Dest {
    fn from(name: &str, number: u32) -> Dest {
        match name {
            "bot" => Dest::Bot(number),
            "output" => Dest::Output(number),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Bot {
    lv: Option<u32>,
    hv: Option<u32>,
    lvd: Option<Dest>,
    hvd: Option<Dest>,
}

impl Bot {
    fn new() -> Bot {
        Bot {
            lv: None,
            hv: None,
            lvd: None,
            hvd: None,
        }
    }
}

fn pass_chip(bots: &mut HashMap<u32, Bot>, number_of_bot: u32, value: u32) {
    if bots.get_mut(&number_of_bot).is_none() {
        bots.insert(number_of_bot, Bot::new());
    }
    if let Some(bot) = bots.get_mut(&number_of_bot) {
        match bot {
            &mut Bot { lv: None, hv: None, .. } => {
                bot.lv = Some(value);
            }
            &mut Bot { lv: Some(x), hv: None, .. } if x < value => {
                bot.hv = Some(value);
            }
            &mut Bot { lv: Some(x), hv: None, .. } if x > value => {
                bot.hv = bot.lv;
                bot.lv = Some(value);
            }
            _ => unreachable!(),
        }
    }
}

fn pass_instructions(bots: &mut HashMap<u32, Bot>, number_of_bot: u32, l: Dest, h: Dest) {
    if bots.get_mut(&number_of_bot).is_none() {
        bots.insert(number_of_bot, Bot::new());
    }
    if let Some(bot) = bots.get_mut(&number_of_bot) {
        bot.lvd = Some(l);
        bot.hvd = Some(h);
    }
    give_chips(bots, number_of_bot);
}

fn give_chips(bots: &mut HashMap<u32, Bot>, number_of_bot: u32) {
    let bot_and_value = match bots.get(&number_of_bot) {
        Some(&Bot { lv: Some(v), hv: Some(..), lvd: Some(Dest::Bot(x)), .. }) => Some((x, v)),
        _ => None,
    };
    if let Some((x, v)) = bot_and_value {
        pass_and_give(bots, x, v);
    }
    let bot_and_value = match bots.get(&number_of_bot) {
        Some(&Bot { lv: Some(..), hv: Some(v), hvd: Some(Dest::Bot(x)), .. }) => Some((x, v)),
        _ => None,
    };
    if let Some((x, v)) = bot_and_value {
        pass_and_give(bots, x, v);
    }
}

fn pass_and_give(bots: &mut HashMap<u32, Bot>, number_of_bot: u32, value: u32) {
    pass_chip(bots, number_of_bot, value);
    give_chips(bots, number_of_bot);
}

fn number_of_bot(instructions: Vec<String>, higher_value: u32, lower_value: u32) -> u32 {
    let mut bots = HashMap::new();
    for instr in instructions {
        let mut tokens = instr.split_whitespace();
        match tokens.next() {
            Some("value") => {
                let value: u32 = tokens.next().unwrap().parse().unwrap();
                let number_of_bot: u32 = tokens.skip(3).next().unwrap().parse().unwrap();
                pass_and_give(&mut bots, number_of_bot, value);
            }
            Some("bot") => {
                let v: Vec<_> = tokens.collect();
                let number_of_bot: u32 = v[0].parse().unwrap();
                pass_instructions(&mut bots,
                                  number_of_bot,
                                  Dest::from(v[4], v[5].parse().unwrap()),
                                  Dest::from(v[9], v[10].parse().unwrap()));
            }
            _ => unreachable!(),
        }
    }
    for (number_of_bot, bot) in bots {
        if let (Some(lv), Some(hv)) = (bot.lv, bot.hv) {
            if lv == lower_value && hv == higher_value {
                return number_of_bot;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(number_of_bot(vec!["value 5 goes to bot 2".to_owned(),
                                      "bot 2 gives low to bot 1 and high to bot 0".to_owned(),
                                      "value 3 goes to bot 1".to_owned(),
                                      "bot 1 gives low to output 1 and high to bot 0".to_owned(),
                                      "bot 0 gives low to output 2 and high to output 0"
                                          .to_owned(),
                                      "value 2 goes to bot 2".to_owned()],
                                 5,
                                 2),
                   2);
    }

}
