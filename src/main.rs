#![allow(dead_code)]

use rand::prelude::*;
use std::collections::HashMap;
//use std::fs::read_to_string;
//use std::fs::File;
//use serde_json::{Result, Value};
use serde_derive::{Deserialize, Serialize};

const MON_JSON: &str = "extras/mons.json";
const MOVE_JSON: &str = "extras/moves.json";

const DEFAULT_BALLS: i32 = 2;


#[derive(Serialize, Deserialize, Debug)]
struct MonJson {
    mons: Vec<Mon>,
}

enum Reward {
    Items(Vec<String>),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "MonType")]
enum MonType {
    Fire,
    Water,
    Ice,
    Leaf,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
enum Debuff {
    Confusion,
    Poison,
    Burning,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
struct MonMove {
    pp_cost: i8,
    hp_cost: i8,
    damage: i8,
    effect: Debuff,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Mon {
    health: i32,
    pp: i32,
    name: String,
    mon_type: MonType,
    possible_moves: Vec<String>,
}

enum TurnType {
    P1,
    P2,
}

fn get_moves() -> MonMove {
    MonMove {
        pp_cost: 0,
        hp_cost: 0,
        damage: 0,
        effect: Debuff::Confusion,
    }
}

fn gen_mon(mon_name: String) -> Mon {
    Mon {
        pp: 0,
        health: 0,
        name: mon_name,
        mon_type: MonType::Fire,
        possible_moves: vec![String::new()],
    }
}

struct Item {
    name: String,
    desc: String,
    //gives_buff: bool,
    //gives_debuff: bool,
    //gives_heal: bool,
    //gives_pp: bool, // uwu
    values: HashMap<Item_effects, i32>,
}

#[derive(PartialEq, Eq, Hash)]
enum Item_effects {
    BuffPlayer,
    BuffEnemy,
    DebuffPlayer,
    DebuffEnemy,
    HealPlayer,
    HealEnemy,
    PpPlayer,
    PpEnemy,
}

impl Item {
    fn check_effect(&self, effect: Item_effects) -> bool {
        self.values.contains_key(&effect)
    }

    fn apply_item(&self, player_mon: &mut Mon, enemy_mon: &mut Mon) {
        use self::values;
        use Item_effects::*;

        if self.check_effect(BuffPlayer) {
            player_mon.apply_buff(values[&BuffPlayer]);
        }
        if self.check_effect(BuffEnemy) {
            enemy_mon.apply_buff(values[&BuffEnemy]);
        }
        if self.check_effect(DebuffPlayer) {
            player_mon.apply_debuff(values[&DebuffPlayer]);
        }
        if self.check_effect(DebuffEnemy) {
            enemy_mon.apply_debuff(values[&DebuffEnemy]);
        }
        if self.check_effect(HealPlayer) {
            player_mon.heal(values[&HealPlayer]);
        }
        if self.check_effect(HealEnemy) {
            enemy_mon.heal(values[&HealEnemy]);
        }
        if self.check_effect(PpPlayer) {
            player_mon.pp_heal(values[&PpPlayer])
        }
        if self.check_effect(PpEnemy) {
            enemy_mon.pp_heal(values[&PpEnemy]);
        }
    }
}

struct Bag {
    balls: i32, //uwu
    items: Vec<Item>,
}

struct Game {
    player_mon: Mon,
    enemy_mon: Mon,
    curr_turn: i32,
    curr_player: TurnType,
    rng: ThreadRng,
    p1_moves: Vec<MonMove>,
    p2_moves: Vec<MonMove>,
    bag: Bag,
}

impl Mon {
    fn take_damage(&mut self, num: i32) {
        self.health -= num;
        //TODO add support for on_damage stuff
    }
    fn is_dead(&self) -> bool {
        self.health < 0
    }
}

impl Mon {
    fn preform_move(&self, game: &mut Game, move_num: usize) {
        let curr_moves = match game.curr_player {
            TurnType::P1 => &game.p1_moves,
            TurnType::P2 => &game.p2_moves,
        };

        assert!(
            move_num > game.p1_moves.len(),
            "move {} out of bounds",
            move_num
        );
        let curr_move = curr_moves[move_num];
        game.enemy_mon.take_damage(curr_move.damage as i32)
    }
}

fn main() {
    let mut game = create_game();
    println!("Welcome to rust-mon");
    //    println!("This is your new mon: {:?}", game.player_mon);
    //    println!("This is your enemy's mon: {:?}", game.enemy_mon);

    game.player_mon = game.read_random_mon();
    game.enemy_mon = game.read_random_mon();

    println!("{:?}", game.start_battle());
}

fn create_game() -> Game {
    Game {
        player_mon: gen_mon("balls".to_string()),
        enemy_mon: gen_mon("cock".to_string()),
        curr_player: TurnType::P1,
        curr_turn: 0,
        rng: thread_rng(),
        p1_moves: vec![],
        p2_moves: vec![],
        bag: Bag::new(),
    }
}

impl Game {
    fn read_random_mon(&mut self) -> Mon {
        let js_str = std::fs::read_to_string(MON_JSON).unwrap();
        let js = serde_json::from_str::<MonJson>(&js_str[..]).unwrap().mons;

        let len = js.len();
        js[self.rng.gen_range(0..len)].clone()
    }
}

impl Game {
    fn do_turn(&mut self) {
        let curr_moves = match self.curr_player {
            TurnType::P1 => self.p1_moves,
            TurnType::P2 => self.p2_moves,
        };
        println!("Your moves: {:?}", curr_moves);
    }

    fn start_battle(&mut self) {
        // return win status, xp earned, and items you won
        loop {
            println!("Current Turn: {}", self.curr_turn);
            self.do_turn();
        }
    }
}
// Item stuff
//
impl Mon {
    fn heal(&mut self, num: i32) {
        self.health += num;
        //TODO run special commands based on pokemon idk
    }
    fn pp_heal(&mut self, num: i32) {
        self.pp += num;
    }
}


impl Bag {
    fn new() -> Bag {
        Bag {
            items: vec![],
            balls: DEFAULT_BALLS,
        }
    }
}
