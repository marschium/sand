use std::collections::HashMap;

use crate::cells::{Cell, Spawner, update_cell};

pub struct CellBlock {
    cells: Vec<Cell>,
    dirty: bool,
    iter_x: i32,
    iter_y: i32,
}

impl CellBlock {

    pub fn new() -> Self {
        let cells = vec![Cell::Air; (REGION_SIZE * REGION_SIZE) as usize];
        CellBlock {
            cells,
            dirty: true,
            iter_x: 0,
            iter_y: 0
        }
    }

    pub fn cells(&self) -> Vec<(&Cell, i32, i32)> {

        fn idx_to_coord(i: usize) -> (i32, i32) {
            let x = i as i32 % REGION_SIZE;
            let y = i as i32 / REGION_SIZE;
            (x, y)
        }

        // TODO don't create a new vector just iterate over existing
        return self.cells
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let c = idx_to_coord(i);
                (x, c.0, c.1)
            })
            .collect();
    }
}

pub struct GameState {
    pub size: i32,
    pub blocks : HashMap<(i32, i32), CellBlock>
}

pub const REGION_SIZE: i32 = 16;

impl GameState {

    pub fn new(size: i32) -> Self {
        let mut blocks = HashMap::new();
        for x in 0..size {
            for y in 0..size {
                blocks.insert((x as i32, y as i32), CellBlock::new());
            }
        }
        GameState {
            size: size * REGION_SIZE,
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

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        let bx = x / REGION_SIZE;
        let by = y / REGION_SIZE;
        let ix = x % REGION_SIZE;
        let iy = y % REGION_SIZE;
        let b = self.blocks.get(&(bx, by)).unwrap();
        b.cells[(ix + (iy * REGION_SIZE)) as usize] == Cell::Air
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

    // run spawners
    // TODO generate once
    let s = Spawner{};
    s.spawn(write_state);

    // reset every block in target
    for (_, block) in write_state.blocks.iter_mut() {
        block.dirty = false;
    }

    for (pos, block) in read_state.blocks.iter() {
        let block_offset = (pos.0 * REGION_SIZE, pos.1 * REGION_SIZE);
        if block.dirty {
            println!("Updating: ({}, {})", pos.0, pos.1);
            for (c, i, j) in block.cells() {
                let world_pos = (i + block_offset.0, j + block_offset.1);
                update_cell(c, world_pos.0, world_pos.1, read_state, write_state);
            }
        }            
    }
}