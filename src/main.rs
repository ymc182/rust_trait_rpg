use std::collections::HashMap;
mod pawn;
mod world;
use pawn::*;
use world::*;
#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Game {
    pub pawns: HashMap<i32, Pawn>,
    pub turn: i32,
    pub map: Map,
}

impl Game {
    pub fn new(size: i32) -> Game {
        Game {
            pawns: HashMap::new(),
            turn: 0,
            map: Map::new(size, size),
        }
    }

    pub fn add_pawn(&mut self, name: String, atk: i32, health: i32, p_type: PawnType) -> i32 {
        let id = self.pawns.len() as i32;
        let pawn = Pawn::new(id, name, atk, health, p_type);
        self.pawns.insert(id, pawn);
        id
    }

    pub fn remove_pawn(&mut self, id: i32) {
        self.pawns.remove(&id);
    }

    pub fn get_pawn(&self, id: i32) -> Option<&Pawn> {
        self.pawns.get(&id)
    }

    pub fn get_pawn_mut(&mut self, id: i32) -> Option<&mut Pawn> {
        self.pawns.get_mut(&id)
    }

    pub fn get_pawn_by_name(&mut self, name: String) -> Option<&mut Pawn> {
        for (_, pawn) in self.pawns.iter_mut() {
            if pawn.name == name {
                return Some(pawn);
            }
        }
        None
    }

    pub fn get_turn(&self) -> i32 {
        self.turn
    }

    pub fn print_map(&self) {
        let pawns_locations: HashMap<(i32, i32), &Pawn> = self
            .pawns
            .iter()
            .map(|(_, pawn)| (pawn.location, pawn))
            .collect();
        println!("Pawn location: {:?}", pawns_locations);

        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if let Some(pawn) = pawns_locations.get(&(x, y)) {
                    print!("[{}]", pawn.name.chars().next().unwrap());
                } else {
                    print!("[ ]");
                }
            }
            println!("");
        }
    }
}

pub fn main() {
    let mut game = Game::new(10).clone();
    let id1 = game.add_pawn("Eric".to_string(), 10, 100, PawnType::Player);
    let id2 = game.add_pawn("Carina".to_string(), 10, 100, PawnType::AI);

    game.move_pawn(id1, 1, 1);
    game.move_pawn(id2, 5, 5);
    game.print_map();
}

pub trait Actions {
    fn turn_check(&mut self, pawn_id: i32) -> bool;
    fn distance(&self, pawn_id: i32, target_id: i32) -> i32;
    fn attack(&mut self, attacker_id: i32, target_id: i32);
    fn move_pawn(&mut self, pawn_id: i32, x: i32, y: i32);
}

impl Actions for Game {
    fn turn_check(&mut self, pawn_id: i32) -> bool {
        if self.turn != pawn_id {
            println!("Not your turn!");
            return false;
        }
        true
    }

    fn distance(&self, pawn_id: i32, target_id: i32) -> i32 {
        let pawn = self.get_pawn(pawn_id).unwrap();
        let target = self.get_pawn(target_id).unwrap();
        let x_dist = (pawn.location.0 - target.location.0).abs();
        let y_dist = (pawn.location.1 - target.location.1).abs();

        x_dist + y_dist
    }

    fn attack(&mut self, attacker_id: i32, target_id: i32) {
        if self.get_pawn(attacker_id).is_none() || self.get_pawn(target_id).is_none() {
            return;
        }

        self.turn_check(attacker_id);
        //TODO: Test
        if self.distance(attacker_id, target_id) > 1 {
            println!("Too far away!");
            return;
        }
        let mut attacker = self.get_pawn(attacker_id).unwrap().clone();
        let target = self.get_pawn_mut(target_id).unwrap();
        attacker.attack(target);
        self.turn += 1 % self.pawns.len() as i32;
    }

    fn move_pawn(&mut self, pawn_id: i32, x: i32, y: i32) {
        self.turn_check(pawn_id);
        let mut pawn = self.get_pawn_mut(pawn_id).unwrap().clone();
        pawn.location = (pawn.location.0 + x, pawn.location.1 + y);
        self.pawns.insert(pawn_id, pawn);
    }
}
