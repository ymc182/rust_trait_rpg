pub mod skill_system;

#[derive(Clone, Debug)]
pub enum SkillType {
    Attack,
    Defense,
    Heal,
}

#[derive(Clone, Debug)]
pub struct Skill {
    pub name: String,
    pub mana_cost: i32,
    pub amount: i32,
    pub skill_type: SkillType,
    pub learnable: bool,
    pub learn_cost: i32,
}
