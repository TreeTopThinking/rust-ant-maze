use crate::assets::Assets;
use raylib::prelude::*;

// The differant textures that can be used
#[derive(Clone, Copy, Debug)]
pub enum Tex {
    Dirt1,
    Dirt2,
    Dirt3,
    Grass1,
    Grass2,
    Grass3,
    Background1,
    Background2,
    Background3,
    Mushroom,
    Rock,
    BushLeft,
    BushRight,
}

impl Tex {
    fn draw_tex(d: &mut RaylibDrawHandle, tex: &Texture2D, pos: Vector2, width: i32) {
        let scale = width as f32 / tex.width as f32;
        d.draw_texture_ex(tex, pos, 0.0, scale, Color::WHITE);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets, x: i32, y: i32, width: i32) {
        let pos = Vector2::new(x as f32, y as f32);

        Tex::draw_tex(d, assets.find(*self), pos, width)
    }
}
