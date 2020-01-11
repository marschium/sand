use rand::prelude::*;

use crate::game::{GameState, REGION_SIZE};

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Air,
    Sand,
}

pub struct RadialSpawner{
    enabled: bool,
    x: i32,
    y: i32,
    deltas: Vec<(i32, i32)>
}

impl RadialSpawner {
    pub fn new(x : i32, y: i32) -> RadialSpawner {
        let deltas = vec! [
            (2,0), (4,0), (6,0),
        (0,2), (2,2), (4,2), (6,2), (8,2),
        (0,4), (2,4), (4,4), (6,4), (8,4),
        (0,6), (2,6), (4,6), (6,6), (8,6),
            (2,8), (4,8), (6,8),
        ];
        RadialSpawner {
            x,
            y,
            deltas: deltas,
            enabled: false
        }
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

pub trait Spawner {

    fn spawn(&mut self, write_state: &mut GameState);

}

impl Spawner for RadialSpawner {

    fn spawn(&mut self, write_state: &mut GameState) {

        if !self.enabled {
            return;
        }

        for (dx, dy) in self.deltas.iter() {
            write_state.write_cell(Cell::Sand, self.x + dx, self.y + dy, true);
        }
    }

}

pub fn update_cell(cell: &Cell, x: i32, y: i32, read_state: &GameState, write_state: &mut GameState) {
    match cell {
        Cell::Sand => {
            let new_y = y + 1;
            let mut sideways = - 1;
            if rand::random() {
                sideways =  1;
            }
            let new_x = x + sideways;
            let height = (read_state.size as i32) - 1;
            if  new_y <= height && read_state.is_empty(x, new_y) && write_state.is_empty(x, new_y) {
                write_state.write_cell(Cell::Sand, x, new_y, true);
                write_state.get_block_mut((x - sideways) / REGION_SIZE, (y - 1) / REGION_SIZE).dirty = true;
            }
            else if new_y <= height && new_x >= 0 && new_x <= height && read_state.is_empty(new_x, new_y)  && write_state.is_empty(new_x, new_y) {
                 write_state.write_cell(Cell::Sand, new_x, new_y, true);                 
                 write_state.get_block_mut((x - sideways) / REGION_SIZE, (y - 1) / REGION_SIZE).dirty = true;
            }
            else {
                write_state.write_cell(Cell::Sand, x, y, false);
            }
        },
        _ => {}
    }
}