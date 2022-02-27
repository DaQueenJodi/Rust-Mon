use crate::*;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::io::{self, Error};

pub struct Game {
    pub player_mon: Mon,
    pub enemy_mon: Mon,
    pub curr_turn: i32,
    pub curr_player: TurnType,
    pub rng: ThreadRng,
    pub p1_moves: Vec<MonMove>,
    pub p2_moves: Vec<MonMove>,
    pub bag: Bag,
}

impl Game {
    pub fn read_specific_mon(name: &str) -> Result<Mon, String> {
        let file = fs::File::open(MON_JSON).expect("Could not open MONS_JSON");
        let js: MonJson = serde_json::from_reader(file).unwrap();
        for i in js.mons {
            if i.name == name {
                return Ok(i);
            }
        }
        return Err("welp".to_owned());
    }
    pub fn read_random_mon(&mut self) -> Mon {
        let js_str = std::fs::read_to_string(MON_JSON).unwrap();
        let js = serde_json::from_str::<MonJson>(&js_str[..]).unwrap().mons;

        let len = js.len();
        js[self.rng.gen_range(0..len)].clone()
    }
}

impl Game {
    pub fn do_turn(&mut self) {
        let curr_moves = match self.curr_player {
            TurnType::P1 => &self.p1_moves,
            TurnType::P2 => &self.p2_moves,
        };
        println!("Your moves: {:?}", curr_moves);
    }

    pub fn start_battle(&mut self) {
        // return win status, xp earned, and items you won

        loop {
            println!("What Mon do you want?: ");
            let x = 1; // start counting at 1
            for i in self.bag.available_mons.iter() {
                print!("{}: {}", x, i.name);
            }
            let prompt = &mut String::new();
            io::stdout().flush().expect("Could not flush buffer");
            io::stdin()
                .read_line(prompt)
                .expect("could not read stdin buffer");
            let prompt: usize = match prompt.trim().parse() {
                Ok(num) => {
                    if num <= self.bag.available_mons.len() {
                        num
                    } else {
                        println!("Not an option!");
                        continue;
                    }
                }

                Err(_) => {
                    println!("Please write a number");
                    continue;
                }
            };

            self.choose_mon(prompt);
            break;
        }
        loop {
            println!("Current Turn: {}", self.curr_turn);

            self.do_turn();
            self.curr_turn += 1;
        }
    }
    pub fn new() -> Game {
        Game {
            player_mon: Mon::new(),
            enemy_mon: Mon::new(),
            curr_player: TurnType::P1,
            curr_turn: 0,
            rng: thread_rng(),
            p1_moves: vec![],
            p2_moves: vec![],
            bag: Bag::new(),
        }
    }
}

impl Game {
    pub fn choose_mon(&mut self, mut num: usize) {
        num -= 1;
        assert!(!(num > self.bag.available_mons.len()));
        let mon = &self.bag.available_mons[num];
        self.player_mon = mon.clone();
    }
}
