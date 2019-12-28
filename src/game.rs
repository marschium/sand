#[derive(Copy, Clone)]
pub enum Cell {
    Air,
    Sand{ delta: i32}
}

pub struct CellBlock {
    cells: Vec<Cell>
}

impl CellBlock {

    pub fn new() -> Self {
        let mut cells = vec![Cell::Air; REGION_SIZE * REGION_SIZE];
        CellBlock {
            cells
        }
    }

    pub fn cells(&self) -> Vec<(Cell, u32, u32)> {
        let mut v = Vec::new();
        for i in 0..REGION_SIZE {
            for j in 0..REGION_SIZE {
                // TODO look in sub regions for ones marked as changed and query them for changed cells
                v.push((self.cells[(i + (j * REGION_SIZE))], i as u32, j as u32));
            }
        }
        v
    }
}

pub struct GameState {
    pub blocks: Vec<CellBlock>,
}

pub const REGION_SIZE: usize = 8;

impl GameState {

    pub fn new() -> Self {
        let mut blocks = vec!();
        blocks.push(CellBlock::new()); // TODO track position
        GameState {
            blocks,
        }
    }

    pub fn reset_block(&mut self, x: u32, y: u32) {
        // TODO use position
        self.blocks = Vec::new();
        self.blocks.push(CellBlock::new());
    }

    pub fn write_cell(&mut self, cell: Cell, x: usize, y: usize) {
        // TODO use block offset
        let mut b = self.blocks.first_mut().unwrap();
        b.cells[(x + (y * REGION_SIZE))] = cell;
    }

    pub fn update(&self, write_state: &mut GameState) {
        // Run the update for every block and write the result
        // Work from the bottom up and update each cell
        // TODO copy unchanged blocks straight over
        // TODO what about updates that span blocks?

        for block in self.blocks.iter() {
            write_state.reset_block(0, 0);
            for (c, i, j) in block.cells() {
                match c {
                    Cell::Sand{delta} => {
                        let nj = j as i32 + delta;
                        let mut d = delta;
                        if nj == 0 {
                            d = 1
                        }
                        if nj == 7 {
                            d = -1
                        }
                        write_state.write_cell(Cell::Sand{delta: d}, i as usize, nj as usize);
                    }
                    _ => {}
                }
            }
        }
    }
}