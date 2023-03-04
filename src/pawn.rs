use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PawnType {
    AI,
    Player,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Pawn {
    pub id: i32,
    pub name: String,
    pub atk: i32,
    pub health: i32,
    pub max_health: i32,
    pub location: (i32, i32),
    pub pawn_type: PawnType,
}

impl Pawn {
    pub fn new(id: i32, name: String, atk: i32, health: i32, p_type: PawnType) -> Pawn {
        Pawn {
            id,
            name,
            atk,
            health,
            max_health: health,
            location: (-1, -1),
            pawn_type: p_type,
        }
    }

    pub fn attack(&mut self, target: &mut Pawn) {
        target.get_damage(self.atk);
    }

    pub fn get_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }
}
