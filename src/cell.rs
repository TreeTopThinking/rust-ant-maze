use crate::assets::Assets;
use crate::tex::Tex;
use crate::tex::Tex::*;
use raylib::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub i: usize,
    pub j: usize,
    pub wall: bool,
    pub tex: Tex,
}

impl Cell {
    pub fn new(i: usize, j: usize) -> Self {
        Cell {
            i: i,
            j: j,
            wall: true,
            tex: Background1,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, assets: &Assets, size: i32) {
        let x = self.i as i32 * size;
        let y = self.j as i32 * size;

        if self.wall {
            self.tex = Dirt1;
        } else {
            self.tex = Background1;
        }

        self.tex.draw(d, assets, x, y, size);
    }
}
