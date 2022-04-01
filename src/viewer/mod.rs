use crate::level::*;

pub struct Viewer {
    level: Level,
    camera_xy: LevelPoint,
}

impl Viewer {
    pub fn new(level: Level) -> Self {
        let camera_xy = level.tiles.rect().center();
        Viewer {
            level,
            camera_xy,
        }
    }

    pub fn move_camera(&mut self, amt: LevelVector) {
        self.camera_xy += amt;
    }

    pub fn render(&self, grid_size: LevelSize, grid: &mut [u32]) {
        let adj_camera_xy = self.camera_xy - grid_size / 2;
        assert_eq!(grid.len() as isize, grid_size.width * grid_size.height);

        let mut index = 0;
        for y in 0..grid_size.height {
            for x in 0..grid_size.width {
                let level_xy = adj_camera_xy + LevelVector::new(x as isize, y as isize);
                let tile = self.level.tiles.get(level_xy);
                let color = match tile {
                    Tile::OutOfBounds => 0x008000,
                    Tile::Floor => 0x000000,
                    Tile::Wall => 0x00FF00,
                };
                grid[index] = color;
                index += 1;
            }
        }
    }
}