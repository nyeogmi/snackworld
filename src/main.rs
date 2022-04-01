mod level;
mod viewer;

use std::collections::HashMap;

use level::{Level, LevelSize, LevelVector, LevelPoint, Tile};
use minifb::{Key, Window, WindowOptions, ScaleMode, Scale, KeyRepeat};
use viewer::Viewer;

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

fn main() {
    let mut opts =  WindowOptions::default();
    opts.scale_mode = ScaleMode::AspectRatioStretch;
    opts.scale = Scale::X8;
    opts.resize = true;
    let mut window = Window::new(
        "snackworld",
        WIDTH, HEIGHT,
        opts,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // loop until the window closes
    let mut viewers = HashMap::<i64, Viewer>::new();
    let mut viewer_ix: i64 = 0;
    let mut buffer = Box::new([0; WIDTH * HEIGHT ]);
    while window.is_open() {
        if window.is_key_down(Key::Escape) { break; }

        if window.is_key_pressed(Key::Left, KeyRepeat::Yes) { viewer_ix -= 1; }
        if window.is_key_pressed(Key::Right, KeyRepeat::Yes) { viewer_ix += 1; }

        let viewer = viewers.entry(viewer_ix)
        .or_insert_with(|| Viewer::new(generate_level(viewer_ix)));

        if window.is_key_pressed(Key::A, KeyRepeat::Yes) { 
            viewer.move_camera(LevelVector::new(-1, 0)); 
        }
        if window.is_key_pressed(Key::D, KeyRepeat::Yes) { 
            viewer.move_camera(LevelVector::new(1, 0)); 
        }
        if window.is_key_pressed(Key::W, KeyRepeat::Yes) { 
            viewer.move_camera(LevelVector::new(0, -1)); 
        }
        if window.is_key_pressed(Key::S, KeyRepeat::Yes) { 
            viewer.move_camera(LevelVector::new(0, 1)); 
        }

        viewer.render(LevelSize::new(WIDTH as isize, HEIGHT as isize), buffer.as_mut());

        window.set_title(&format!("snackworld - seed={}", viewer_ix));
        window.update_with_buffer(buffer.as_ref(), WIDTH, HEIGHT).unwrap();
    }
}

fn generate_level(seed: i64) -> Level {
    let mut level = Level::new();

    let radius = seed as isize;

    if radius < 0 { return level; }

    for x in -radius..=radius {
        for y in -radius..=radius {
            let mut tile = Tile::Floor;
            if x.abs() == radius || y.abs() == radius {
                tile = Tile::Wall;
            }
            level.tiles.set(LevelPoint::new(x, y), tile);
        }
    }
    return level;
}