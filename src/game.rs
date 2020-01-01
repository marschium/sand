use std::collections::HashMap;

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
    pub blocks : HashMap<(u32, u32), CellBlock>
}

pub const REGION_SIZE: usize = 8;

impl GameState {

    pub fn new() -> Self {
        let mut blocks = HashMap::new();
        blocks.insert((0, 0), CellBlock::new());
        blocks.insert((1, 0), CellBlock::new());
        blocks.insert((0, 1), CellBlock::new());
        blocks.insert((1, 1), CellBlock::new());
        GameState {
            blocks,

        }
    }

    pub fn reset_block(&mut self, x: u32, y: u32) {
        // TODO index blocks based on global x/y instead of block x/y?
        self.blocks.insert((x, y), CellBlock::new());
    }

    pub fn write_cell(&mut self, cell: Cell, x: usize, y: usize) {
        // TODO if not there, create block
        let bx = (x / REGION_SIZE) as u32;
        let by = (y / REGION_SIZE) as u32;
        let ix = x % REGION_SIZE;
        let iy = y % REGION_SIZE;
        let b = self.blocks.get_mut(&(bx,by)).unwrap();
        b.cells[(ix + (iy * REGION_SIZE))] = cell;
    }

    pub fn update(&self, write_state: &mut GameState) {
        // Run the update for every block and write the result
        // Work from the bottom up and update each cell
        // TODO copy unchanged blocks straight over
        // TODO what about updates that span blocks?

        for (pos, block) in self.blocks.iter() {
            write_state.reset_block(pos.0, pos.1);
            let block_offset = (pos.0 * REGION_SIZE as u32, pos.1 * REGION_SIZE as u32);
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
                        write_state.write_cell(Cell::Sand{delta: d}, (i + block_offset.0) as usize, (nj + block_offset.1 as i32) as usize);
                    }
                    _ => {}
                }
            }
        }
    }
}