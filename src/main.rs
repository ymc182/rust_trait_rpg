use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Pawn {
    pub id: i32,
    pub name: String,
    pub atk: i32,
    pub health: i32,
    pub max_health: i32,
    pub location: (i32, i32),
}

impl Pawn {
    pub fn new(id: i32, name: String, atk: i32, health: i32) -> Pawn {
        Pawn {
            id,
            name,
            atk,
            health,
            max_health: health,
            location: (-1, -1),
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
#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Game {
    pub pawns: HashMap<i32, Pawn>,
    pub turn: i32,
    pub map: Map,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let mut tiles = Vec::new();
        for x in 0..width {
            let mut row = Vec::new();
            for y in 0..height {
                row.push(Tile { x, y });
            }
            tiles.push(row);
        }
        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(&self.tiles[x as usize][y as usize])
    }

    pub fn get_tile_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(&mut self.tiles[x as usize][y as usize])
    }
}

impl Game {
    pub fn new(size: i32) -> Game {
        Game {
            pawns: HashMap::new(),
            turn: 0,
            map: Map::new(size, size),
        }
    }

    pub fn add_pawn(&mut self, name: String, atk: i32, health: i32) -> i32 {
        let id = self.pawns.len() as i32;
        let pawn = Pawn::new(id, name, atk, health);
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
    pub fn attack(&mut self, attacker_id: i32, target_id: i32) {
        if self.get_pawn(attacker_id).is_none() || self.get_pawn(target_id).is_none() {
            return;
        }

        if self.turn != attacker_id {
            println!("Not your turn!");
            return;
        }

        let mut attacker = self.get_pawn(attacker_id).unwrap().clone();
        let target = self.get_pawn_mut(target_id).unwrap();
        attacker.attack(target);
        self.turn += 1 % self.pawns.len() as i32;
    }

    pub fn get_turn(&self) -> i32 {
        self.turn
    }

    pub fn move_pawn(&mut self, pawn_id: i32, x: i32, y: i32) {
        let mut pawn = self.get_pawn_mut(pawn_id).unwrap().clone();
        pawn.location = (pawn.location.0 + x, pawn.location.1 + y);
        self.pawns.insert(pawn_id, pawn);
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
    let id1 = game.add_pawn("Eric".to_string(), 10, 100);
    let id2 = game.add_pawn("Carina".to_string(), 10, 100);

    game.move_pawn(id1, 1, 1);
    game.move_pawn(id2, 5, 5);

    game.print_map();
}
