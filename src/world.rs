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
