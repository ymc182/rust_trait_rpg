use crate::character::Pawn;

use super::*;

impl Skill {
    pub fn new(
        name: String,
        mana_cost: i32,
        amount: i32,
        skill_type: SkillType,
        learnable: bool,
        learn_cost: i32,
    ) -> Skill {
        Skill {
            name,
            mana_cost,
            amount,
            skill_type,
            learnable,
            learn_cost,
        }
    }

    pub fn learn(&self, pawn: &mut Pawn, slot: usize) {
        if pawn.exp < self.learn_cost {
            println!("You don't have enough exp to learn this skill!");
            return;
        }
        if (pawn.skills_slots[slot].is_some()) {
            println!(
                "Are you sure you want to replace {} with {}? (y/n)",
                pawn.skills_slots[slot].as_ref().unwrap().name,
                self.name
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() != "y" {
                return;
            }
        }
        pawn.exp -= self.learn_cost;
        pawn.skills_slots[slot] = Some(self.clone());
    }
}
