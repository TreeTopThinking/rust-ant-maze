use ::rand::random_range;
use macroquad::prelude::*;
use std::ops::Range;

#[derive(Clone, Copy)]
pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
}

impl Circle {
    pub fn new(x: Range<f32>, y: Range<f32>) -> Self {
        let radius = random_range(10.0..50.0);

        Circle {
            x: random_range((x.start + radius)..(x.end - radius)),
            y: random_range((y.start + radius)..(y.end - radius)),
            radius: radius,
            color: Color::from_rgba(
                10,
                random_range(120..170),
                random_range(230..255),
                random_range(10..50),
            ),
        }
    }

    fn draw(&self, radius: f32) {
        draw_circle(self.x, self.y, radius, self.color);
    }

    pub fn draw_blurry(&self) {
        for offset in (0..20).step_by(4) {
            let radius = self.radius + offset as f32;
            self.draw(radius);
        }
    }
}
