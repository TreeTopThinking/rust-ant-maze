use crate::assets::Assets;
use macroquad::prelude::*;

// The differant textures that can be used
#[derive(Clone, Copy, Debug)]
pub enum TexKind {
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

impl TexKind {
    fn draw_tex(tex: &Texture2D, pos: Vec2, size: i32) {
        draw_texture_ex(
            tex,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size as f32, size as f32)),
                ..Default::default()
            },
        );
    }

    pub fn draw(&self, assets: &Assets, x: i32, y: i32, width: i32) {
        let pos = vec2(x as f32, y as f32);

        TexKind::draw_tex(assets.find(*self), pos, width)
    }
}
