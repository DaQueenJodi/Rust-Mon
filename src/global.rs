use crate::*;
use int_enum::IntEnum;
use serde_derive::{Deserialize, Serialize};
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, IntEnum)]
pub enum Debuff {
    Nothing = 0,
    Confusion = 1,
    Poison = 2,
    Burning = 3,
}
#[repr(i32)]
#[derive(Copy, Debug, PartialEq, Eq, Clone, IntEnum, Serialize, Deserialize)]
pub enum Buff {
    Nothing = 0,
    BetterHeal = 1,
    Speed = 2,
    Damage = 3,
}
pub enum TurnType {
    P1,
    P2,
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub enum Immunities {
    Debuff(Debuff),
    Buff(Buff),
    MonType(MonType),
}
