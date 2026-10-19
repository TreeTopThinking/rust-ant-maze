use crate::cell::Cell;
use crate::player::Player;
use crate::{COLS, ROWS};
use ::rand::random_range;
use macroquad::prelude::*;

// Hazards and Collectibles have a ton of identical methods so eventually they the common functionality should turn into a trait
pub struct Hazards<'a> {
    rects: Vec<Rect>,
    tex: &'a Texture2D,
}

impl<'a> Hazards<'a> {
    pub fn new(tex: &'a Texture2D, grid: &[[Cell; ROWS]; COLS], size: i32) -> Self {
        const SCALE: f32 = 0.03;

        let mut hazards = Hazards {
            rects: Vec::new(),
            tex: tex,
        };

        for i in 0..COLS {
            for j in 0..ROWS {
                if grid[i][j].artifact.is_none()
                    && !grid[i][j].wall
                    && j < COLS - 1
                    && grid[i][j + 1].wall
                    && (i, j) != (COLS - 2, ROWS - 2)
                    && random_range(0.0..1.0) < 0.2
                {
                    let w = tex.width() * SCALE;
                    let h = tex.height() * SCALE;

                    let rect = Rect::new(
                        i as f32 * size as f32 + random_range(0.0..(size as f32 - w)),
                        j as f32 * size as f32 + size as f32 - h,
                        w,
                        h,
                    );

                    hazards.rects.push(rect);
                }
            }
        }

        hazards
    }

    pub fn colliding(&self, player: &Player<COLS, ROWS>) -> bool {
        for rect in &self.rects {
            // Makes the hitbox more forgiving
            if player.rectangle_collision(rect.x, rect.y + 7.0, rect.w, rect.h - 7.0) {
                return true;
            }
        }

        return false;
    }

    pub fn draw(&self) {
        for rect in &self.rects {
            draw_texture_ex(
                self.tex,
                rect.x,
                rect.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(rect.w, rect.h)),
                    ..Default::default()
                },
            );
        }
    }
}
