use crate::global::*;
use crate::*;
use int_enum::{IntEnum, IntEnumError};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ItemEffects {
    BuffPlayer,
    BuffEnemy,
    DebuffPlayer,
    DebuffEnemy,
    HealPlayer,
    HealEnemy,
    PpPlayer,
    PpEnemy,
}

pub struct Item {
    name: String,
    desc: String,
    //gives_buff: bool,
    //gives_debuff: bool,
    //gives_heal: bool,
    //gives_pp: bool, // uwu
    values: HashMap<ItemEffects, i32>,
}

impl Item {
    fn check_effect(&self, effect: ItemEffects) -> bool {
        self.values.contains_key(&effect)
    }

    fn apply_item(&self, player_mon: &mut Mon, enemy_mon: &mut Mon) {
        let values = &self.values;
        use ItemEffects::*;

        if self.check_effect(BuffPlayer) {
            let num = values[&BuffPlayer];
            player_mon.apply_buff(Buff::from_int(num).unwrap());
        }
        if self.check_effect(BuffEnemy) {
            let num = values[&BuffEnemy];
            enemy_mon.apply_buff(Buff::from_int(num).unwrap());
        }
        if self.check_effect(DebuffPlayer) {
            player_mon.apply_debuff(Debuff::from_int(values[&DebuffPlayer]).unwrap());
        }
        if self.check_effect(DebuffEnemy) {
            let num = values[&DebuffEnemy];
            enemy_mon.apply_debuff(Debuff::from_int(num).unwrap());
        }
        if self.check_effect(HealPlayer) {
            player_mon.heal(values[&HealPlayer]);
        }
        if self.check_effect(HealEnemy) {
            enemy_mon.heal(values[&HealEnemy]);
        }
        if self.check_effect(PpPlayer) {
            player_mon.pp_heal(values[&PpPlayer]);
        }
        if self.check_effect(PpEnemy) {
            enemy_mon.pp_heal(values[&PpEnemy]);
        }
    }
}

pub struct Bag {
    pub balls: i32, //uwu
    pub items: Vec<Item>,
    pub available_mons: Vec<Mon>,
}

impl Bag {
    pub fn new() -> Bag {
        Bag {
            items: vec![],
            available_mons: vec![Game::read_specific_mon("Balls").unwrap()],
            balls: DEFAULT_BALLS,
        }
    }
}
