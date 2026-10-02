use crate::cell::Cell;
use macroquad::{
    audio::{PlaySoundParams, Sound, play_sound},
    prelude::*,
};
use quad_gif::GifAnimation;

#[derive(PartialEq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player<const COLS: usize, const ROWS: usize> {
    pub pos: Vec2,
    pub size: Vec2,
    walking_size: Vec2,
    climbing_size: Vec2,
    vel: Vec2,
    acc_y: f32,
    jump: f32,
    jumped: bool,
    player_speed: f32,
    climb_speed: f32,
    cells: [[Cell; COLS]; ROWS],
    cell_size: f32,
    is_on_floor: bool,
    direction: Dir,
    climbing: bool,
}

impl<const COLS: usize, const ROWS: usize> Player<COLS, ROWS> {
    pub fn new(
        cells: [[Cell; COLS]; ROWS],
        cell_size: f32,
        x: f32,
        y: f32,
        walking_width: f32,
        walking_height: f32,
        gravity: f32,
    ) -> Self {
        Player {
            pos: vec2(x, y),
            size: vec2(walking_width, walking_height),
            walking_size: vec2(walking_width, walking_height),
            climbing_size: vec2(walking_height, walking_width),
            vel: vec2(0.0, 0.0),
            acc_y: gravity,
            jump: 0.9,
            jumped: false,
            player_speed: 0.8,
            climb_speed: 0.8,
            cells: cells,
            cell_size: cell_size,
            is_on_floor: false,
            direction: Dir::Left,
            climbing: false,
        }
    }

    pub fn update(&mut self) {
        self.climbing = false;

        self.vel.x = 0.0;

        let mut left_pressed = false;
        let mut right_pressed = false;

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.vel.x -= self.player_speed;
            left_pressed = true;

            self.direction = Dir::Left;
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.vel.x += self.player_speed;
            right_pressed = true;

            self.direction = Dir::Right;
        }

        self.pos.x += self.vel.x;

        if let Some((i, j)) = self.check_collision() {
            let (x, _y, width, _height) = self.get_cell_info(i, j);

            if self.vel.x > 0.0 {
                if right_pressed && self.vel.y > 0.0 {
                    self.vel.y = 0.0;
                }

                if right_pressed {
                    self.climbing = true;
                }

                self.pos.x = x - self.size.x;
            } else {
                if left_pressed && self.vel.y > 0.0 {
                    self.vel.y = 0.0;
                }

                if left_pressed {
                    self.climbing = true;
                }

                self.pos.x = x + width;
            }
        }

        if self.climbing {
            self.size.y = self.climbing_size.y;
        } else {
            self.size = self.walking_size;
        }

        if (is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::W)
            || is_key_pressed(KeyCode::Up))
            && !self.jumped
        {
            self.vel.y -= self.jump;
            self.jumped = true;
        }

        if self.climbing
            && (is_key_down(KeyCode::Space) || is_key_down(KeyCode::W) || is_key_down(KeyCode::Up))
        {
            self.vel.y = -self.climb_speed;
        }

        self.vel.y += self.acc_y;
        self.pos.y += self.vel.y;

        self.is_on_floor = false;
        if let Some((i, j)) = self.check_collision() {
            let (_x, y, _width, height) = self.get_cell_info(i, j);

            let move_up = self.pos.y + self.size.y - y;
            let move_down = y + height - self.pos.y;

            if move_up < move_down {
                self.pos.y -= move_up;
                self.is_on_floor = true;
                self.jumped = false;
            } else {
                self.pos.y += move_down;
            }

            self.vel.y = 0.0;
        }

        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
        }
    }

    pub fn get_i_j(&self) -> (i32, i32) {
        let i = (self.pos.x as f32 / self.cell_size as f32).floor() as i32;
        let j = (self.pos.y as f32 / self.cell_size as f32).floor() as i32;

        (i, j)
    }

    fn get_cell_info(&self, i: i32, j: i32) -> (f32, f32, f32, f32) {
        let x = i as f32 * self.cell_size;
        let y = j as f32 * self.cell_size;
        let width = self.cell_size;
        let height = self.cell_size;

        (x, y, width, height)
    }

    fn rectangle_collision(
        &self,
        rect_x: f32,
        rect_y: f32,
        rect_width: f32,
        rect_height: f32,
    ) -> bool {
        let player_left = self.pos.x;
        let player_right = self.pos.x + self.size.x;
        let player_top = self.pos.y;
        let player_bottom = self.pos.y + self.size.y;
        let rect_left = rect_x;
        let rect_right = rect_x + rect_width;
        let rect_top = rect_y;
        let rect_bottom = rect_y + rect_height;

        player_left < rect_right
            && player_right > rect_left
            && player_top < rect_bottom
            && player_bottom > rect_top
    }

    // Returns None if no collisions are found
    // Returns (i, j) if colliding
    pub fn check_collision(&self) -> Option<(i32, i32)> {
        let (i, j) = self.get_i_j();

        for dx in -1..=1 {
            for dy in -1..=1 {
                let check_i = i + dx;
                let check_j = j + dy;
                if check_i >= COLS as i32 || check_i < 0 || check_j >= ROWS as i32 || check_j < 0 {
                    continue;
                }

                let (x, y, width, height) = self.get_cell_info(check_i, check_j);
                let cell = self.cells[check_i as usize][check_j as usize];

                if cell.wall && self.rectangle_collision(x, y, width, height) {
                    return Some((check_i, check_j));
                }
            }
        }

        None
    }

    fn draw_tex(&self, tex: &Texture2D) {
        let draw_size = if self.climbing {
            self.climbing_size
        } else {
            self.walking_size
        };

        let mut draw_pos = self.pos;
        draw_pos.y += self.size.y - draw_size.y;

        if self.climbing && self.direction == Dir::Right {
            draw_pos.x += self.size.x - draw_size.x;
        }

        draw_texture_ex(
            &tex,
            draw_pos.x,
            draw_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(draw_size),
                ..Default::default()
            },
        )
    }

    fn animate_gif(&self, gif: &mut GifAnimation) {
        self.draw_tex(&gif.frame().texture);
        gif.tick();
    }

    pub fn draw(
        &self,
        left: &Texture2D,
        right: &Texture2D,
        mut left_walk: &mut GifAnimation,
        mut right_walk: &mut GifAnimation,
        left_idle_climb: &Texture2D,
        right_idle_climb: &Texture2D,
        mut left_climb: &mut GifAnimation,
        mut right_climb: &mut GifAnimation,
    ) {
        if self.direction == Dir::Left && !self.climbing && self.vel.x != 0.0 {
            self.animate_gif(&mut left_walk);
        } else if self.direction == Dir::Right && !self.climbing && self.vel.x != 0.0 {
            self.animate_gif(&mut right_walk);
        } else if self.direction == Dir::Left && self.climbing && self.vel.y < 0.0 {
            self.animate_gif(&mut left_climb);
        } else if self.direction == Dir::Right && self.climbing && self.vel.y < 0.0 {
            self.animate_gif(&mut right_climb);
        } else if self.direction == Dir::Left && !self.climbing {
            self.draw_tex(left);
        } else if self.direction == Dir::Right && !self.climbing {
            self.draw_tex(right);
        } else if self.direction == Dir::Left && self.climbing {
            self.draw_tex(left_idle_climb);
        } else if self.direction == Dir::Right && self.climbing {
            self.draw_tex(right_idle_climb);
        }
    }
    //
    //     pub fn play_sounds(&self, walking_sound: &Sound, jump_sound: &Sound) {
    //         if self.climbing {
    //             play_sound(
    //                 walking_sound,
    //                 PlaySoundParams {
    //                     looped: false,
    //                     volume: 0.5,
    //                 },
    //             );
    //         }
    //     }
}
