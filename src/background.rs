use crate::assets::Assets;
use crate::background;
use crate::tex::TexKind;
use ::rand::random_range;
use macroquad::prelude::*;

pub struct Background<const COLS: usize, const ROWS: usize> {
    grid: [[TexKind; COLS]; ROWS],
}

impl<const COLS: usize, const ROWS: usize> Background<COLS, ROWS> {
    pub fn new() -> Self {
        let mut background = Background {
            grid: [[TexKind::Background1; COLS]; ROWS],
        };

        for i in 0..COLS {
            for j in 0..ROWS {
                let n = random_range(0..3);

                let tile: TexKind;

                if j == 0 && n == 0 {
                    tile = TexKind::Grass1;
                } else if j == 0 && n == 1 {
                    tile = TexKind::Grass2;
                } else if j == 0 && n == 2 {
                    tile = TexKind::Grass3;
                } else if n == 0 {
                    tile = TexKind::Dirt1;
                } else if n == 1 {
                    tile = TexKind::Dirt2;
                } else {
                    tile = TexKind::Dirt3;
                }

                background.grid[i][j] = tile;
            }
        }

        background
    }

    fn draw_tex(assets: &Assets, tex: TexKind, pos: Vec2, size: i32) {
        tex.draw(assets, pos.x as i32, pos.y as i32, size);
    }

    pub fn draw(&self, assets: &Assets, size: i32) {
        for i in 0..COLS {
            for j in 0..ROWS {
                let x = i as i32 * size - (size * 5);
                let y = j as i32 * size;
                Self::draw_tex(&assets, self.grid[i][j], vec2(x as f32, y as f32), size);
            }
        }
    }
}
