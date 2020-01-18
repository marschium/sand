use std::collections::HashMap;
use sdl2::render::Texture;
use sdl2::rect::Rect;

use crate::cells::{Cell, Spawner, update_cell};
use crate::render;

pub struct CellBlock {
    cells: Vec<Cell>,
    pub dirty: bool,
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

pub struct GameState<'a> {
    pub size: i32,
    pub blocks : HashMap<(i32, i32), CellBlock>,
    texture: Texture<'a>,
}

pub const REGION_SIZE: i32 = 16;

impl<'a> GameState<'a> {

    pub fn new(size: i32, texture: Texture<'a>) -> Self {
        let mut blocks = HashMap::new();
        for x in 0..size {
            for y in 0..size {
                blocks.insert((x as i32, y as i32), CellBlock::new());
            }
        }
        GameState {
            size: size * REGION_SIZE,
            blocks,
            texture,
        }
    }

    pub fn get_block_mut(&mut self, bx: i32, by: i32) -> Option<&mut CellBlock> {
        self.blocks.get_mut(&(bx, by))
    }

    pub fn mark_block_dirty(&mut self, x: i32, y: i32) {
        match self.get_block_mut((x) / REGION_SIZE, (y) / REGION_SIZE) {
            Some(b) => {
                b.dirty = true;
            },
            None => {}
        }
    }

    pub fn reset_block(&mut self, x: i32, y: i32) {
        // TODO index blocks based on global x/y instead of block x/y?
        let r = Rect::new(x * REGION_SIZE,  y * REGION_SIZE, REGION_SIZE as u32, REGION_SIZE as u32);
        self.texture.update(r, &vec![0u8; 16 * 16 * 24], 16 * 3);
        self.blocks.insert((x, y), CellBlock::new());

        let tx = x * REGION_SIZE;
        let ty = y * REGION_SIZE;
        let i = tx + (ty * REGION_SIZE);

    }

    pub fn get_tex(&mut self) -> &mut Texture<'a> {
        &mut self.texture
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        let bx = x / REGION_SIZE;
        let by = y / REGION_SIZE;
        let ix = x % REGION_SIZE;
        let iy = y % REGION_SIZE;
        match self.blocks.get(&(bx, by)) {
            Some(b) => {
                if ix < 0 || ix >= REGION_SIZE || y < 0 || iy >= REGION_SIZE {
                    return false;
                }
                return b.cells[(ix + (iy * REGION_SIZE)) as usize] == Cell::Air
            },
            None => {
                return false;
            }
        }
    }

    pub fn read_cell(&self, x: i32, y: i32) -> &Cell {
        let bx = x / REGION_SIZE;
        let by = y / REGION_SIZE;
        let ix = x % REGION_SIZE;
        let iy = y % REGION_SIZE;
        match self.blocks.get(&(bx, by)) {
            Some(b) => {
                if ix < 0 || ix >= REGION_SIZE || y < 0 || iy >= REGION_SIZE {
                    return &Cell::Air; // Maybe a magic enum for boundary?
                }
                &b.cells[(ix + (iy * REGION_SIZE)) as usize]
            },
            None => {
                &Cell::Air // Maybe a magic enum for boundary?
            }
        }
    }

    pub fn write_cell(&mut self, cell: Cell, x: i32, y: i32, dirty: bool) {
        let bx = x / REGION_SIZE;
        let by = y / REGION_SIZE;
        let ix = x % REGION_SIZE;
        let iy = y % REGION_SIZE;

        if ix < 0 || ix >= REGION_SIZE || iy < 0 || iy >= REGION_SIZE {
            return;
        }

        match self.get_block_mut(bx,by) {
            Some(b) => {
                if dirty {
                    b.dirty = dirty;
                }
                b.cells[(ix + (iy * REGION_SIZE)) as usize] = cell;
                match cell {
                    Cell::Air => {},
                    _ => {
                        let r = Rect::new( x, y, 1, 1);
                        let c = render::get_cell_color(cell);
                        self.texture.update(r, &vec![c.b, c.g, c.r], 3); // TODO set position and color
                    }
                }
            },
            None => {}
        }         
    }
}

pub fn update(read_state: &GameState, write_state: &mut GameState, spawner: &mut impl Spawner) {

    // clear any blocks that will be changed
    // copy any blocks that won't
    for (pos, block) in read_state.blocks.iter() {
        if block.dirty {
            // println!("Updating: ({},{})", pos.0, pos.1);
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
    
    spawner.spawn(write_state); // TODO FIX TRYING TO SET OUTSIDE WORLD

    for (pos, block) in read_state.blocks.iter() {
        let block_offset = (pos.0 * REGION_SIZE, pos.1 * REGION_SIZE);
        if block.dirty {
            for (c, i, j) in block.cells() {
                let world_pos = (i + block_offset.0, j + block_offset.1);
                update_cell(c, world_pos.0, world_pos.1, read_state, write_state);
            }
        }            
    }
}