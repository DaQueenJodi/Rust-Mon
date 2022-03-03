use crate::global::*;
use crate::*;
use core::fmt;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct MonMove {
    pub name: String,
    pub pp_cost: i8,
    pub hp_cost: i8,
    pub damage: i8,
    pub effect: Debuff,
}

impl fmt::Display for MonMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: Costs {} HP and {} PP. Deals {} damage and gives the enemy {}",
            self.name, self.hp_cost, self.pp_cost, self.damage, self.effect
        )
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "MonType")]
pub enum MonType {
    Fire,
    Water,
    Ice,
    Leaf,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mon {
    pub health: i32,
    pub max_health: i32,
    pub pp: i32,
    pub max_pp: i32,
    pub name: String,
    pub mon_type: MonType,
    pub possible_moves: Vec<String>,
    pub immunities: Vec<Immunities>,
    pub moves: Vec<MonMove>,
    pub level: i32,
    pub debuffs: Vec<Debuff>,
    pub buffs: Vec<Buff>,
}

impl Mon {
    pub fn take_damage(&mut self, num: i32) {
        self.health -= num;
        //TODO add support for on_damage stuff
    }
    pub fn is_dead(&self) -> bool {
        self.health < 0
    }
}

pub fn get_moves() -> MonMove {
    MonMove {
        name: String::new(),
        pp_cost: 0,
        hp_cost: 0,
        damage: 0,
        effect: Debuff::Confusion,
    }
}

impl Mon {
    pub fn preform_move(&self, game: &mut Game, move_num: usize) {
        let curr_moves = match game.curr_player {
            TurnType::P1 => &game.p1_moves,
            TurnType::P2 => &game.p2_moves,
        };

        assert!(
            move_num > game.p1_moves.len(),
            "move {} out of bounds",
            move_num
        );
        let curr_move = &curr_moves[move_num];
        game.enemy_mon.take_damage(curr_move.damage as i32)
    }
}

impl Mon {
    pub fn heal(&mut self, num: i32) {
        self.health += num;
        //TODO run special commands based on pokemon idk
    }
    pub fn pp_heal(&mut self, num: i32) -> bool {
        if self.pp < self.max_pp {
            self.pp += num;
            true
        } else {
            false
        }
    }
    pub fn apply_debuff(&mut self, debuff: Debuff) -> bool {
        if !self.immunities.contains(&Immunities::Debuff(debuff)) {
            self.debuffs.push(debuff);
            true
        } else {
            false
        }
    }
    pub fn apply_buff(&mut self, buff: Buff) -> bool {
        if !self.immunities.contains(&Immunities::Buff(buff)) {
            self.buffs.push(buff);
            true
        } else {
            false
        }
    }
}
impl Mon {
    pub fn new() -> Mon {
        Mon {
            buffs: vec![],
            debuffs: vec![],
            moves: vec![],
            level: 0,
            immunities: vec![],
            max_health: 0,
            max_pp: 0,
            pp: 0,
            health: 0,
            name: String::new(),
            mon_type: MonType::Fire,
            possible_moves: vec![String::new()],
        }
    }
}

impl Mon {
    pub fn level_up(&mut self) {
        self.level += 1;
        self.learn_move_auto();
    }

    pub fn learn_move_auto(&mut self) {

        //println!("{:?}", self.possible_moves);
        //println!("{}", self.possible_moves[0].clone());

        let move_name = match self.level {
        1 => self.possible_moves[0].clone(),
        5 => self.possible_moves[1].clone(),
        10 => self.possible_moves[2].clone(),
        _ => "None".to_owned(),
        };

        if !(move_name == "None".to_owned()) {
            self.learn_move(Game::read_specific_move(move_name.trim().to_owned()).unwrap())
        }
    }
    pub fn learn_move(&mut self, mon_move: MonMove) {
        self.moves.push(mon_move);
    }
}
