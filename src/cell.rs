use crate::assets::Assets;
use crate::tex::TexKind;
use crate::tex::TexKind::*;
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub i: usize,
    pub j: usize,
    pub wall: bool,
    pub tex: TexKind,
    pub artifact: Option<TexKind>,
}

impl Cell {
    pub fn new(i: usize, j: usize) -> Self {
        Cell {
            i: i,
            j: j,
            wall: true,
            tex: Background1,
            artifact: None,
        }
    }

    pub fn draw(&self, assets: &Assets, size: i32) {
        let x = self.i as i32 * size;
        let y = self.j as i32 * size;

        self.tex.draw(assets, x, y, size);
        if self.artifact.is_some() {
            self.artifact.unwrap().draw(assets, x, y, size);
        }
    }
}
