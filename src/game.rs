use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum Cell {
    Air,
    Sand{ delta: (i32, i32)},
    OtherSand,
}

pub struct CellBlock {
    cells: Vec<Cell>,
    dirty: bool
}

impl CellBlock {

    pub fn new() -> Self {
        let mut cells = vec![Cell::Air; (REGION_SIZE * REGION_SIZE) as usize];
        CellBlock {
            cells,
            dirty: true
        }
    }

    pub fn cells(&self) -> Vec<(Cell, i32, i32)> {
        let mut v = Vec::new();
        for i in 0..REGION_SIZE {
            for j in 0..REGION_SIZE {
                // TODO look in sub regions for ones marked as changed and query them for changed cells
                v.push((self.cells[(i + (j * REGION_SIZE)) as usize], i, j));
            }
        }
        v
    }
}

pub struct GameState {
    pub blocks : HashMap<(i32, i32), CellBlock>
}

pub const REGION_SIZE: i32 = 8;

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

    pub fn reset_block(&mut self, x: i32, y: i32) {
        // TODO index blocks based on global x/y instead of block x/y?
        self.blocks.insert((x, y), CellBlock::new());
    }

    pub fn write_cell(&mut self, cell: Cell, x: i32, y: i32, dirty: bool) {
        let bx = x / REGION_SIZE;
        let by = y / REGION_SIZE;
        let ix = x % REGION_SIZE;
        let iy = y % REGION_SIZE;
        let b = self.blocks.get_mut(&(bx,by)).unwrap();
        if dirty {
            b.dirty = dirty;
        }
        b.cells[(ix + (iy * REGION_SIZE)) as usize] = cell;
    }
}

pub fn update(read_state: &GameState, write_state: &mut GameState) {

    // clear any blocks that will be changed
    // copy any blocks that won't
    for (pos, block) in read_state.blocks.iter() {
        if block.dirty {
            write_state.reset_block(pos.0, pos.1);
        }
        else{
            // copy before any potential updates. so that updates from other blocks into this one aren't lost
            write_state.blocks.get_mut(&(pos.0, pos.1)).unwrap().cells = block.cells.clone();
        }
    }

    // reset every block in target
    for (_, block) in write_state.blocks.iter_mut() {
        block.dirty = false;
    }

    for (pos, block) in read_state.blocks.iter() {
        let block_offset = (pos.0 * REGION_SIZE, pos.1 * REGION_SIZE);
        if block.dirty {
            println!("Updating: ({}, {})", pos.0, pos.1);
            for (c, i, j) in block.cells() {
                // TODO start from the bottom
                let world_pos = (i + block_offset.0, j + block_offset.1);
                match c {
                    Cell::Sand{delta} => {
                        let updated_world_pos = (world_pos.0 + delta.0, world_pos.1 + delta.1);
                        let mut d = delta;
                        if updated_world_pos == (0, 0) {
                            d = (1, 0)
                        }
                        if updated_world_pos == ((REGION_SIZE * 2) - 1, 0) {
                            d = (0, 1)
                        }
                        if updated_world_pos == ((REGION_SIZE * 2) - 1, (REGION_SIZE * 2) - 1) {
                            d = (-1, 0)
                        }
                        if updated_world_pos == (0, (REGION_SIZE * 2) - 1) {
                            d = (0, -1)
                        }
                        write_state.write_cell(Cell::Sand{delta: d}, updated_world_pos.0, updated_world_pos.1, true);
                    },
                    Cell::OtherSand => {
                        write_state.write_cell(Cell::OtherSand, world_pos.0, world_pos.1, false);
                    }
                    _ => {}
                }
            }
        }            
    }
}