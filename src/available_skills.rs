use crate::skill_system::{Skill, SkillType};

pub fn generate_skills() -> Vec<Skill> {
    let mut skills = Vec::new();
    skills.push(Skill::new(
        "Fireball".to_string(),
        10,
        15,
        SkillType::Attack,
        true,
        0,
    ));
    skills.push(Skill::new(
        "Heal".to_string(),
        15,
        10,
        SkillType::Heal,
        true,
        10,
    ));
    skills.push(Skill::new(
        "Shield".to_string(),
        15,
        10,
        SkillType::Defense,
        true,
        10,
    ));
    skills
}
