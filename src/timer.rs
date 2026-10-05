use macroquad::prelude::*;

pub struct Timer {
    start_time: f32,
    pub best_time: f32,
    pub time: f32,
    count: u32,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: get_time() as f32,
            best_time: f32::INFINITY,
            time: 0.0,
            count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.start_time = get_time() as f32;
        self.time = 0.0;
        self.count = 0;
    }

    pub fn update(&mut self) {
        self.time = get_time() as f32 - self.start_time;
    }

    pub fn draw_time(&self) {
        let str_timer = format!("{:.1}", self.time);
        draw_text(str_timer, 10.0, screen_height() - 10.0, 50.0, WHITE);
    }

    pub fn blink_time(&mut self) {
        if self.count % 40 < 20 {
            self.draw_time();
        }

        self.count += 1;
    }

    pub fn update_best(&mut self) {
        if self.time < self.best_time {
            self.best_time = self.time;
        }
    }

    pub fn draw_best(&self) {
        let str_best = format!("Best: {:.1}", self.best_time);
        draw_text(str_best, 150.0, screen_height() - 10.0, 50.0, ORANGE);
    }
}
