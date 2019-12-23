#[derive(Copy, Clone)]
pub enum Cell {
    Air,
    Sand
}


pub struct GameState {
    cells: Vec<Cell>
}

const REGION_SIZE: usize = 8;

impl GameState {

    pub fn new() -> Self {
        let mut cells = vec![Cell::Air; REGION_SIZE * REGION_SIZE];
        for i in 0..REGION_SIZE {
            cells[i] = Cell::Sand;
        }
        GameState {
            cells
        }
    }

    pub fn update(&mut self) {

    }

    pub fn changed_cells(&self) -> Vec<(Cell, i32, i32)> {
        let mut v = Vec::new();
        for i in 0..REGION_SIZE {
            for j in 0..REGION_SIZE {
                // TODO look in sub regions for ones marked as changed and query them for changed cells
                v.push((self.cells[(i + (j * REGION_SIZE))], i as i32, j as i32));
            }
        }
        v
    }
}