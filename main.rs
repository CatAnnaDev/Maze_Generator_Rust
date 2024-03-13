use rand::{thread_rng, Rng, rngs::ThreadRng};

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[derive(Clone, Copy)]
struct Cell {
    col: usize,
    row: usize,
}

impl Cell {
    fn from(col: usize, row: usize) -> Self { Self {col, row} }
}

struct Maze {
    cells: [[bool; HEIGHT]; WIDTH],
    walls_h: [[bool; WIDTH]; HEIGHT + 1],
    walls_v: [[bool; WIDTH + 1]; HEIGHT],
    thread_rng: ThreadRng,
}

impl Maze {

    fn new() -> Self {
        Self {
            cells: [[true; HEIGHT]; WIDTH],
            walls_h: [[true; WIDTH]; HEIGHT + 1],
            walls_v: [[true; WIDTH + 1]; HEIGHT],
            thread_rng: thread_rng(),
        }
    }

    fn first(&mut self) -> Cell {
        Cell::from(self.thread_rng.gen_range(0..WIDTH), self.thread_rng.gen_range(0..HEIGHT))
    }

    fn remove_wall(&mut self, cell1: &Cell, cell2: &Cell) {
        if cell1.row == cell2.row { self.walls_v[cell1.row][if cell1.col > cell2.col { cell1.col } else { cell2.col }] = false; }
        else{ self.walls_h[if cell1.row > cell2.row { cell1.row } else { cell2.row }][cell1.col] = false; }
    }

    fn neighbor(&mut self, cell: &Cell) -> Option<Cell> {
        self.cells[cell.col][cell.row] = false;
        let mut neighbors = Vec::new();
        if cell.col > 0 && self.cells[cell.col - 1][cell.row] { neighbors.push(Cell::from(cell.col - 1, cell.row)); }
        if cell.row > 0 && self.cells[cell.col][cell.row - 1] { neighbors.push(Cell::from(cell.col, cell.row - 1)); }
        if cell.col < WIDTH - 1 && self.cells[cell.col + 1][cell.row] { neighbors.push(Cell::from(cell.col + 1, cell.row)); }
        if cell.row < HEIGHT - 1 && self.cells[cell.col][cell.row + 1] { neighbors.push(Cell::from(cell.col, cell.row + 1)); }
        if neighbors.is_empty() { None }
        else {
            let next = neighbors.get(self.thread_rng.gen_range(0..neighbors.len())).unwrap();
            self.remove_wall(cell, next);
            Some(*next)
        }
    }

    fn build(&mut self) -> &mut Maze {
        let mut cell_stack: Vec<Cell> = Vec::new();
        let mut next = self.first();
        loop {
            while let Some(cell) = self.neighbor(&next) {
                cell_stack.push(cell);
                next = cell;
            }
            match cell_stack.pop() {
                Some(cell) => next = cell,
                None => break,
            }
        }

        self
    }

    fn paint_wall(h_wall: bool, active: bool) {
        if h_wall { print!("{}", if active { "+---" } else { "+   " }); }
        else{ print!("{}", if active { "|   " } else { "    " }); }
    }

    fn paint_close_wall(h_wall: bool) {
        if h_wall { println!("+") }
       else{ println!() }
    }

    fn paint_row(&self, h_walls: bool, index: usize) {
        let iter = if h_walls { self.walls_h[index].iter() } else { self.walls_v[index].iter() };
        for &wall in iter {
            Self::paint_wall(h_walls, wall);
        }
        Self::paint_close_wall(h_walls);
    }

    fn paint(&self) {
        for i in 0 .. HEIGHT {
            self.paint_row(true, i);
            self.paint_row(false, i);
        }
        self.paint_row(true, HEIGHT);
    }
}

fn main() {
    Maze::new().build().paint();
}