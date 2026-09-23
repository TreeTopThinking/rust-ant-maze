use crate::assets::Assets;
use crate::cell::Cell;
use raylib::prelude::RaylibDrawHandle;

// Depth first search maze generator
pub struct DfsMazeGen<const COLS: usize, const ROWS: usize> {
    cells: [[Cell; COLS]; ROWS],
    current: Cell,
    stack: Vec<Cell>,
}

impl<const COLS: usize, const ROWS: usize> DfsMazeGen<{ COLS }, { ROWS }> {
    pub fn new() -> Self {
        let mut cells = [[Cell::new(0, 0); COLS]; ROWS];

        for i in 0..COLS as usize {
            for j in 0..ROWS as usize {
                cells[i][j] = Cell::new(i, j);
            }
        }

        DfsMazeGen {
            cells: cells,
            current: Cell::new(1, 1),
            stack: Vec::new(),
        }
    }

    fn in_bounds(i: i32, j: i32) -> bool {
        i >= 0 && j >= 0 && i < COLS as i32 && j < ROWS as i32
    }

    fn pick_neighbor(&mut self) -> Option<Cell> {
        let mut neighbors = vec![];

        let up = (self.current.i as i32, self.current.j as i32 - 2);
        let down = (self.current.i as i32, self.current.j as i32 + 2);
        let left = (self.current.i as i32 - 2, self.current.j as i32);
        let right = (self.current.i as i32 + 2, self.current.j as i32);

        if Self::in_bounds(up.0, up.1) && self.cells[up.0 as usize][up.1 as usize].wall {
            neighbors.push(self.cells[up.0 as usize][up.1 as usize]);
        }
        if Self::in_bounds(down.0, down.1) && self.cells[down.0 as usize][down.1 as usize].wall {
            neighbors.push(self.cells[down.0 as usize][down.1 as usize]);
        }
        if Self::in_bounds(left.0, left.1) && self.cells[left.0 as usize][left.1 as usize].wall {
            neighbors.push(self.cells[left.0 as usize][left.1 as usize]);
        }
        if Self::in_bounds(right.0, right.1) && self.cells[right.0 as usize][right.1 as usize].wall
        {
            neighbors.push(self.cells[right.0 as usize][right.1 as usize]);
        }

        if neighbors.len() > 0 {
            Some(neighbors[rand::random_range(0..neighbors.len())])
        } else {
            self.stack.pop();
            None
        }
    }

    fn move_current(&mut self) {
        let neighbor = self.pick_neighbor();

        if neighbor.is_some() {
            let mut last = self.cells[self.current.i][self.current.j];
            let mut next = neighbor.unwrap();
            let mut between = self.cells[(last.i + next.i) / 2][(last.j + next.j) / 2];

            last.wall = false;
            between.wall = false;
            next.wall = false;

            self.cells[last.i][last.j] = last;
            self.cells[between.i][between.j] = between;
            self.cells[next.i][next.j] = next;

            self.current = next;
            self.stack.push(next);
        }
    }

    fn finished(&self) -> bool {
        for i in (1..COLS as usize).step_by(2) {
            for j in (1..ROWS as usize).step_by(2) {
                if self.cells[i][j].wall {
                    return false;
                }
            }
        }

        true
    }

    pub fn generate(&mut self) -> [[Cell; COLS]; ROWS] {
        loop {
            if !self.finished() {
                self.move_current();

                if self.stack.len() > 0 {
                    self.current = self.stack[self.stack.len() - 1];
                }
            } else {
                break self.cells;
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, assets: &Assets, size: i32) {
        for row in self.cells {
            for mut cell in row {
                cell.draw(d, assets, size);
            }
        }
    }
}
