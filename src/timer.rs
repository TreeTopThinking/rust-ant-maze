use macroquad::prelude::*;

pub struct Timer {
    start_time: f32,
    timer: f32,
    count: u32,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: get_time() as f32,
            timer: 0.0,
            count: 0,
        }
    }

    pub fn update(&mut self) {
        self.timer = get_time() as f32 - self.start_time;
    }

    pub fn draw(&self) {
        let str_timer = format!("{:.1}", self.timer);
        draw_text(str_timer, 10.0, screen_height() - 10.0, 50.0, WHITE);
    }

    pub fn blink(&mut self) {
        if self.count % 40 < 20 {
            self.draw();
        }

        self.count += 1;
    }
}
