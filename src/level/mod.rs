use euclid::*;
use gridd_euclid::CopyEndlessGrid;

pub struct Level {
    // TODO: multilayer?
    pub tiles: CopyEndlessGrid<Tile, LevelSpace>,
}

pub struct LevelSpace;
pub type LevelPoint = Point2D<isize, LevelSpace>;
pub type LevelSize = Size2D<isize, LevelSpace>;
pub type LevelVector = Vector2D<isize, LevelSpace>;

#[derive(Clone, Copy)]
pub enum Tile {
    OutOfBounds,
    Floor,
    Wall, 
}

impl Level {
    pub fn new() -> Self {
        Self {
            tiles: CopyEndlessGrid::new(Tile::OutOfBounds),
        }
    }
}