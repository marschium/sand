use std::collections::HashMap;
use sdl2::render::Texture;
use sdl2::rect::Rect;

use crate::cells::{Cell, Spawner, update_cell};
use crate::render;

pub struct CellBlock {
    cells: HashMap<(i32, i32), Cell>,
    pub dirty: bool,
}

impl CellBlock {

    pub fn new() -> Self {
        let cells = HashMap::new();
        CellBlock {
            cells,
            dirty: true,
        }
    }

    fn set_cell(&mut self, cell: Cell, x: i32, y: i32) {        
        self.cells.insert((x, y), cell);
    }

    fn get_cell(&self, x: i32, y: i32) -> &Cell {
        match self.cells.get(&(x, y)) {
            Some(c) => {
                &c
            },
            None => {
                &Cell::Air
            }
        }
    }

    fn clear(&mut self) {
        self.cells.clear();
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
        let block_num = size / REGION_SIZE;
        for x in 0..block_num {
            for y in 0..block_num {
                blocks.insert((x as i32, y as i32), CellBlock::new());
            }
        }
        GameState {
            size,
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
        self.texture.update(r, &vec![0u8; 16 * 16 * 24], 16 * 24);
        self.blocks.get_mut(&(x, y)).unwrap().clear();
    }

    pub fn get_tex(&mut self) -> &mut Texture<'a> {
        &mut self.texture
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        let bx = x / REGION_SIZE;
        let by = y / REGION_SIZE;
        let ix = x % REGION_SIZE;
        let iy = y % REGION_SIZE;

        if ix < 0 || ix >= REGION_SIZE || iy < 0 || iy >= REGION_SIZE {
            return false;
        }

        match self.blocks.get(&(bx, by)) {
            Some(b) => {
                return b.get_cell(ix, iy) == &Cell::Air;
                //return b.cells[(ix + (iy * REGION_SIZE)) as usize] == Cell::Air
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

        if ix < 0 || ix >= REGION_SIZE || iy < 0 || iy >= REGION_SIZE {
            return &Cell::Air; // Maybe a magic enum for boundary?
        }

        match self.blocks.get(&(bx, by)) {
            Some(b) => {
                b.get_cell(ix, iy)
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
                b.set_cell(cell, ix, iy);
                //b.cells[(ix + (iy * REGION_SIZE)) as usize] = cell;
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

    pub fn clear(&mut self) {        
        for (_, block) in self.blocks.iter_mut() {
            block.clear();
        }
        let r = Rect::new(0,  0, render::MAP_SIZE as u32, render::MAP_SIZE as u32);
        self.texture.update(r, &vec![0u8; (render::MAP_SIZE * render::MAP_SIZE * 24) as usize], 16 * 24);
    }
}

pub fn update(read_state: &GameState, write_state: &mut GameState, spawner: &mut impl Spawner) {

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
            for ((i, j), c) in block.cells.iter() {
                let world_pos = (i + block_offset.0, j + block_offset.1);
                update_cell(c.clone(), world_pos.0, world_pos.1, read_state, write_state);
            }
        }            
    }
    
    spawner.spawn(write_state);
}
