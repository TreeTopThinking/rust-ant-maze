use raylib::ffi::Rectangle;
use raylib::prelude::*;

// Draws the menu
pub struct Menu<const WIDTH: i32, const HEIGHT: i32> {
    left: Rectangle,
    right: Rectangle,
}

impl<const WIDTH: i32, const HEIGHT: i32> Menu<WIDTH, HEIGHT> {
    pub fn new(offset: i32) -> Self {
        Menu {
            left: Rectangle::new(
                offset as f32,
                (offset + 200) as f32,
                (WIDTH / 2 - offset) as f32,
                (HEIGHT - 2 * offset - 200) as f32,
            ),
            right: Rectangle::new(
                (WIDTH / 2 + offset) as f32,
                (offset + 200) as f32,
                (WIDTH / 2 - 2 * offset) as f32,
                (HEIGHT - 2 * offset - 200) as f32,
            ),
        }
    }

    pub fn draw_menu(&self, d: &mut RaylibDrawHandle) {
        d.draw_text("Mole Maze", 50, 50, 175, Color::WHITE);
        d.draw_rectangle_lines_ex(self.left, 5.0, Color::DARKBROWN);
        d.draw_rectangle_lines_ex(self.right, 5.0, Color::DARKBROWN);
    }

    pub fn choice(rl: &mut RaylibHandle) {
        let mouse_x = rl.get_mouse_x();
        let mouse_y = rl.get_mouse_y();
    }
}
