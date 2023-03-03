use super::*;
pub mod action;
pub mod character;
use skill_system::Skill;

#[derive(Clone, Debug)]
pub struct Pawn {
    pub name: String,
    pub max_health: i32,
    pub max_mana: i32,
    pub health: i32,
    pub mana: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub shield: i32,
    pub exp: i32,
    pub skills_slots: [Option<Skill>; 4],
}
