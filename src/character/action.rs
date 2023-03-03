use std::cmp::{max, min};

use crate::skill_system::SkillType;

use super::*;

pub trait Actions {
    fn attack(&mut self, target: &mut Pawn);
    fn defend(&mut self);
    fn cast(&mut self, target: &mut Pawn, slot: usize);
    fn be_attacked(&mut self, attacker: &mut Pawn, raw_damage: i32);
}

impl Actions for Pawn {
    fn attack(&mut self, target: &mut Pawn) {
        target.be_attacked(self, self.strength);
    }

    fn defend(&mut self) {
        self.shield = (self.dexterity + self.strength) / 2;
    }

    fn cast(&mut self, target: &mut Pawn, slot: usize) {
        if let Some(skill) = &self.skills_slots[slot] {
            if self.mana < skill.mana_cost {
                println!("You don't have enough mana to cast this skill!");
                return;
            }
            self.mana -= skill.mana_cost;
            match skill.skill_type {
                SkillType::Attack => {
                    println!("{} casted {}!", self.name, skill.name);
                    target.be_attacked(self, skill.amount);
                }
                SkillType::Heal => {
                    self.health = min(self.max_health, self.health + skill.amount);
                    println!(
                        "{} healed {} for {} health! ({})",
                        self.name, self.name, skill.amount, self.health
                    );
                }
                SkillType::Defense => {
                    self.shield += skill.amount;
                    println!(
                        "{} casted {} for {} shield!",
                        self.name, skill.name, skill.amount
                    );
                }
            }
        }
    }

    fn be_attacked(&mut self, attacker: &mut Pawn, raw_damage: i32) {
        println!(
            "{} is attacking {} for {} damage!",
            attacker.name, self.name, raw_damage
        );
        let damage = max(0, raw_damage - self.shield);
        if self.shield > 0 {
            println!("{}'s shield absorbed {} damage!", self.name, self.shield);
            self.shield = max(0, self.shield - raw_damage);
        }
        if damage > 0 {
            self.health -= damage;
            println!("{} took {} damage! ({})", self.name, damage, self.health);
        }

        if self.health <= 0 {
            attacker.gain_exp(self.max_health);
            println!("{} died!", self.name);
        }
    }
}
