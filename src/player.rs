use crate::cell::Cell;
use macroquad::prelude::*;
use quad_gif::GifAnimation;

#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player<const COLS: usize, const ROWS: usize> {
    pub pos: Vec2,
    pub size: Vec2,
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
        width: f32,
        height: f32,
        gravity: f32,
    ) -> Self {
        Player {
            pos: vec2(x, y),
            size: vec2(width, height),
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

        if is_key_down(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            self.vel.y += 0.01;
        }

        self.pos.x += self.vel.x;

        if let Some((i, j)) = self.check_collision() {
            let (x, _y, width, _height) = self.get_cell_info(i, j);

            if self.vel.x > 0.0 {
                self.pos.x = x - self.size.x;

                if right_pressed && self.vel.y > 0.0 {
                    self.vel.y = 0.0;
                }

                if right_pressed {
                    self.climbing = true;
                }
            } else {
                self.pos.x = x + width;

                if left_pressed && self.vel.y > 0.0 {
                    self.vel.y = 0.0;
                }

                if left_pressed {
                    self.climbing = true;
                }
            }
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
            self.direction = Dir::Up;
            self.vel.y = -self.climb_speed;
        }

        self.vel.y += self.acc_y;
        self.pos.y += self.vel.y;

        self.is_on_floor = false;
        if let Some((i, j)) = self.check_collision() {
            let (_x, y, _width, height) = self.get_cell_info(i, j);
            if self.vel.y > 0.0 {
                self.pos.y = y - self.size.y;
                self.is_on_floor = true;
                self.jumped = false;
            } else {
                self.pos.y = y + height;
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
        draw_texture_ex(
            &tex,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.size.x, self.size.y)),
                ..Default::default()
            },
        )
    }

    pub fn draw(
        &self,
        left: &Texture2D,
        right: &Texture2D,
        left_walk: &Texture2D,
        right_walk: &Texture2D,
        left_idle_climb: &Texture2D,
       right_idle_climb: &Texture2D,
       left_climb: &GifAnimation,
       right_climb: &GifAnimation,

    ),
     {
        if self.direction == Dir::Left {
            self.draw_tex(left);
        } else if self.direction == Dir::Right {
            self.draw_tex(right);
        } else if self.direction == Dir::Up {
            self.draw_tex(up);
        }
    }
}
