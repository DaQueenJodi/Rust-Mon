#![allow(dead_code)]

pub mod game;
pub mod global;
pub mod item;
pub mod mon;
use crate::game::*;
use crate::global::*;
use crate::item::*;
use crate::mon::*;
use rand::prelude::*;
use std::collections::HashMap;

use std::{error::Error, fmt};

//use std::fs::read_to_string;
//use std::fs::File;
//use serde_json::{Result, Value};
use serde_derive::{Deserialize, Serialize};

const MON_JSON: &str = "extras/mons.json";
const MOVE_JSON: &str = "extras/moves.json";

const DEFAULT_BALLS: i32 = 2;

#[derive(Serialize, Deserialize, Debug)]
pub struct MonJson {
    pub mons: Vec<Mon>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MoveJson {
    pub moves: Vec<MonMove>,
}

impl fmt::Display for MonJson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not properly parse json")
    }
}

//enum Reward {
//    Items(Vec<String>),
//}
fn main() {
    let mut game = Game::new();
    game.bag.available_mons[0].learn_move_auto();
    println!("Welcome to rust-mon");
    //    println!("This is your new mon: {:?}", game.player_mon);
    //    println!("This is your enemy's mon: {:?}", game.enemy_mon);

    game.player_mon = game.read_random_mon();
    game.enemy_mon = game.read_random_mon();

    println!("{:?}", game.start_battle());
}
