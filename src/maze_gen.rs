use crate::ab_gen::AbMazeGen;
use crate::assets::Assets;
use crate::cell::Cell;
use crate::dfs_gen::DfsMazeGen;
use crate::tex::TexKind;
use rand::random_range;
use raylib::prelude::*;

pub trait MazeGen<const COLS: usize, const ROWS: usize> {
    fn get_mut_cells(&mut self) -> &mut [[Cell; COLS]; ROWS];
    fn get_cells(&self) -> [[Cell; COLS]; ROWS];

    fn is_wall(&self, i: usize, j: usize) -> bool;

    fn in_bounds(i: i32, j: i32) -> bool {
        i >= 0 && j >= 0 && i < COLS as i32 && j < ROWS as i32
    }

    fn finished(&self) -> bool {
        for i in (1..COLS as usize).step_by(2) {
            for j in (1..ROWS as usize).step_by(2) {
                if self.get_cells()[i][j].wall {
                    return false;
                }
            }
        }

        true
    }

    fn set_tex(&mut self) {
        for i in 0..COLS {
            for j in 0..ROWS {
                let wall = self.is_wall(i, j);
                let grass =
                    wall && Self::in_bounds(i as i32, j as i32 - 1) && !self.is_wall(i, j - 1);
                let add_artifact =
                    !wall && Self::in_bounds(i as i32, j as i32 + 1) && self.is_wall(i, j + 1);
                let left = Self::in_bounds(i as i32 - 1, j as i32) && self.is_wall(i - 1, j);
                let right = Self::in_bounds(i as i32 + 1, j as i32) && self.is_wall(i + 1, j);
                let n = random_range(0..3);
                let cells = &mut self.get_mut_cells();
                let cell = &mut cells[i][j];

                if grass {
                    if n < 1 {
                        cell.tex = TexKind::Grass1;
                    } else if n < 2 {
                        cell.tex = TexKind::Grass2
                    } else {
                        cell.tex = TexKind::Grass3
                    }
                } else if !wall {
                    if n < 1 {
                        cell.tex = TexKind::Background1;
                    } else if n < 2 {
                        cell.tex = TexKind::Background2;
                    } else {
                        cell.tex = TexKind::Background3;
                    }

                    if add_artifact {
                        let artifact_n = random_range(0..10);

                        if left {
                            if artifact_n < 1 {
                                cell.artifact = Some(TexKind::BushLeft);
                            }
                        } else if right {
                            if artifact_n < 2 {
                                cell.artifact = Some(TexKind::BushRight);
                            }
                        } else {
                            if artifact_n < 3 {
                                cell.artifact = Some(TexKind::Mushroom);
                            } else if artifact_n < 4 {
                                cell.artifact = Some(TexKind::Rock);
                            }
                        }
                    }
                } else {
                    if n < 1 {
                        cell.tex = TexKind::Dirt1;
                    } else if n < 2 {
                        cell.tex = TexKind::Dirt2;
                    } else {
                        cell.tex = TexKind::Dirt3;
                    }
                }
            }
        }
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle, assets: &Assets, size: i32) {
        for row in self.get_cells() {
            for cell in row {
                cell.draw(d, assets, size);
            }
        }
    }
}

impl<const COLS: usize, const ROWS: usize> MazeGen<COLS, ROWS> for DfsMazeGen<COLS, ROWS> {
    fn get_mut_cells(&mut self) -> &mut [[Cell; COLS]; ROWS] {
        &mut self.cells
    }

    fn get_cells(&self) -> [[Cell; COLS]; ROWS] {
        self.cells
    }

    fn is_wall(&self, i: usize, j: usize) -> bool {
        self.cells[i][j].wall
    }
}

impl<const COLS: usize, const ROWS: usize> MazeGen<COLS, ROWS> for AbMazeGen<COLS, ROWS> {
    fn get_mut_cells(&mut self) -> &mut [[Cell; COLS]; ROWS] {
        &mut self.cells
    }

    fn get_cells(&self) -> [[Cell; COLS]; ROWS] {
        self.cells
    }

    fn is_wall(&self, i: usize, j: usize) -> bool {
        self.cells[i][j].wall
    }
}
