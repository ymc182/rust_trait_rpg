use super::*;
impl Pawn {
    pub fn new(
        name: String,
        health: i32,
        mana: i32,
        strength: i32,
        dexterity: i32,
        intelligence: i32,
    ) -> Pawn {
        Pawn {
            name,
            max_health: health,
            max_mana: mana,
            health,
            mana,
            strength,
            dexterity,
            intelligence,
            shield: 0,
            exp: 0,
            skills_slots: [None, None, None, None],
        }
    }

    pub fn gain_exp(&mut self, exp: i32) {
        println!("{} gained {} exp!", self.name, exp);
        self.exp += exp;
    }
}
