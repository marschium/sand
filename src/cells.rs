use rand::prelude::*;

use crate::game::{GameState, REGION_SIZE};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Cell {
    Air,
    Sand,
    Wood{fuel: i32},
    Fire{heat: i32},
    Seed,
    Vine{growth: i32, grown: bool},
}

pub struct RadialSpawner{
    enabled: bool,
    x: i32,
    y: i32,
    deltas: Vec<(i32, i32)>,
    cell: Cell
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
            enabled: false,
            cell: Cell::Sand
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

    pub fn set_cell(&mut self, cell: Cell) {
        self.cell = cell
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
            write_state.write_cell(self.cell, self.x + dx, self.y + dy, true);
        }
    }

}

pub fn random_axis(a: i32) -> i32 {
    let mut da = a;
    if rand::random() {
        da = da - 1;
    }
    if rand::random() {
        da = da + 1;
    }
    da
}

pub fn random_dir(x: i32, y: i32) -> (i32, i32) {
    (random_axis(x), random_axis(y))
}

pub fn update_cell(cell: &Cell, x: i32, y: i32, read_state: &GameState, write_state: &mut GameState) {
    match cell {
        Cell::Air => {}
        Cell::Sand => {
            let _ = gravity(Cell::Sand, x, y, read_state, write_state);
        },
        Cell::Wood{fuel} => {
            if fuel <= &0 {                
                write_state.write_cell(Cell::Air, x, y, false);
                return;
            }
            let (dx, dy) = random_dir(x, y);
            match read_state.read_cell(dx, dy) {
                Cell::Fire{..} => {
                     write_state.write_cell(Cell::Air, x, y, true);
                     write_state.write_cell(Cell::Fire{heat: 30}, x, y, true);
                },
                _ => {
                    write_state.write_cell(Cell::Wood{fuel: fuel.clone()}, x, y, false);
                }
            }
        },
        Cell::Fire{heat} => {
            if heat <= &0 {
                write_state.write_cell(Cell::Air, x, y, false);
                return;
            }
            let (dx, dy) = random_dir(x, y);
            match read_state.read_cell(dx, dy) {
                Cell::Air => {
                    let mut degrade = 1;
                    if rand::random() {
                        degrade = 2;
                    }
                    write_state.write_cell(Cell::Fire{heat: heat - degrade}, dx, dy, true);
                }
                _ => {
                    write_state.write_cell(Cell::Fire{heat: heat - 1}, x, y, true);
                }
            }
        },
        Cell::Seed => {
            match gravity(Cell::Seed, x, y, read_state, write_state) {
                GravityResult::OnGround => {
                    match read_state.read_cell(x, y + 1) {
                        Cell::Sand => {
                            write_state.write_cell(Cell::Vine{growth: 50, grown: false}, x, y, true);
                        },
                        _ => {}
                    }
                },
                GravityResult::Falling => {}
            }
        },
        Cell::Vine{growth, grown} => {
            write_state.write_cell(Cell::Vine{growth: *growth, grown: *grown}, x, y, !*grown);

            if *growth <= 0 || *grown {
                return;
            }
            

            let (dx, mut dy) = random_dir(x, y);
            if dy > y {
                dy = y;
            }  
            match read_state.read_cell(dx, dy) {
                Cell::Air => {                  
                    write_state.write_cell(Cell::Vine{growth: growth - 1, grown: false}, dx, dy, true);
                    write_state.write_cell(Cell::Vine{growth: growth.clone(), grown: true}, x, y, true);
                },
                _ => {}
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum GravityResult {
    OnGround,
    Falling
}

fn gravity(cell: Cell, x: i32, y: i32, read_state: &GameState, write_state: &mut GameState) ->  GravityResult{
    let new_y = y + 1;
    let mut sideways = - 1;
    if rand::random() {
        sideways =  1;
    }
    let new_x = x + sideways;
    let height = (read_state.size as i32) - 1;
    if  new_y <= height && read_state.is_empty(x, new_y) && write_state.is_empty(x, new_y) {
        write_state.write_cell(cell, x, new_y, true);
        write_state.get_block_mut((x - sideways) / REGION_SIZE, (y - 1) / REGION_SIZE).dirty = true;
        GravityResult::Falling
    }
    else if new_y <= height && new_x >= 0 && new_x <= height && read_state.is_empty(new_x, new_y)  && write_state.is_empty(new_x, new_y) {
         write_state.write_cell(cell, new_x, new_y, true);                 
         write_state.get_block_mut((x - sideways) / REGION_SIZE, (y - 1) / REGION_SIZE).dirty = true;
         GravityResult::Falling
    }
    else {
        write_state.write_cell(cell, x, y, false);
        GravityResult::OnGround
    }
}