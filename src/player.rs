use crate::cell::Cell;
use raylib::{ffi::Rectangle, prelude::*};

#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player<const COLS: usize, const ROWS: usize> {
    pub pos: Vector2,
    pub size: Vector2,
    vel: Vector2,
    acc_y: f32,
    jump: f32,
    player_speed: f32,
    cells: [[Cell; COLS]; ROWS],
    cell_size: f32,
    is_on_floor: bool,
    jump_count: i32,
    direction: Dir,
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
            pos: Vector2::new(x, y),
            size: Vector2::new(width, height),
            vel: Vector2::new(0.0, 0.0),
            acc_y: gravity,
            jump: 0.1,
            player_speed: 0.03,
            cells: cells,
            cell_size: cell_size,
            is_on_floor: false,
            jump_count: 0,
            direction: Dir::Left,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, width: i32) {
        if (rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            || rl.is_key_pressed(KeyboardKey::KEY_W)
            || rl.is_key_pressed(KeyboardKey::KEY_UP))
            && self.jump_count < 2
        {
            self.vel.y -= self.jump;
            self.jump_count += 1;
        }

        self.vel.x = 0.0;

        let mut left_pressed = false;
        let mut right_pressed = false;

        if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT) {
            if self.pos.x > 0.0 {
                self.vel.x -= self.player_speed;
                left_pressed = true;
            }

            self.direction = Dir::Left;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            if self.pos.x < width as f32 - self.size.x {
                self.vel.x += self.player_speed;
                right_pressed = true;
            }

            self.direction = Dir::Right;
        }

        if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            self.vel.y += self.player_speed;
        }

        self.pos.x += self.vel.x;

        if let Some((i, j)) = self.check_collision() {
            let (x, _y, width, _height) = self.get_cell_info(i, j);

            if self.vel.x > 0.0 {
                self.pos.x = x - self.size.x;

                if right_pressed && self.vel.y > 0.0 {
                    self.vel.y = 0.0;
                }
            } else {
                self.pos.x = x + width;

                if left_pressed && self.vel.y > 0.0 {
                    self.vel.y = 0.0;
                }
            }
        }

        self.vel.y += self.acc_y;
        self.pos.y += self.vel.y;

        self.is_on_floor = false;
        if let Some((i, j)) = self.check_collision() {
            let (_x, y, _width, height) = self.get_cell_info(i, j);
            if self.vel.y > 0.0 {
                self.pos.y = y - self.size.y;
                self.is_on_floor = true;
                self.jump_count = 0;
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

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>, left: &Texture2D, right: &Texture2D) {
        let source = Rectangle::new(0.0, 0.0, left.width as f32, left.height as f32);
        let destination = Rectangle::new(self.pos.x, self.pos.y, self.size.x, self.size.y);

        if self.direction == Dir::Left {
            d.draw_texture_pro(
                left,
                source,
                destination,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        } else {
            d.draw_texture_pro(
                right,
                source,
                destination,
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
    }
}
