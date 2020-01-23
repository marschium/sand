use rand::prelude::*;

use crate::game::{GameState, REGION_SIZE};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Cell {
    Air,
    Sand,
    Wood,
    Fire{heat: i32},
    Seed,
    Vine{growth: i32, grown: bool},
    Water{dx: i32},
    Acid{t: i32}
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
            (0,0), (1,0), (2,0), (3,0), (4,0), (5,0), (6,0),
            (0,1), (1,1), (2,1), (3,1), (4,1), (5,1), (6,1),
        (0,2), (1,2), (2,2), (3,2), (4,2), (5,2), (6,2), (7,2), (8,2),
        (0,3), (1,3), (2,3), (3,3), (4,3), (5,3), (6,3), (7,3), (8,3),
        (0,4), (1,4), (2,4), (3,4), (4,4), (5,4), (6,4), (7,4), (8,4),
        (0,5), (1,5), (2,5), (3,5), (4,5), (5,5), (6,5), (7,5), (8,5),
        (0,6), (1,6), (2,6), (3,6), (4,6), (5,6), (6,6), (7,6), (8,6),
            (0,7), (1,7), (2,7), (3,7), (4,7), (5,7), (6,7),
            (0,8), (1,8), (2,8), (3,8), (4,8), (5,8), (6,8),
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

pub fn update_cell(cell: Cell, x: i32, y: i32, read_state: &GameState, write_state: &mut GameState) {
    
    if !write_state.is_empty(x, y) {
        return;
    }

    match cell {
        Cell::Air => {},
        Cell::Sand => {
            if dissolve_in_acid(x, y, read_state, write_state) == AcidResult::Dissolved {
                return;
            }
            let _ = gravity(Cell::Sand, x, y, read_state, write_state);
        },
        Cell::Wood => {
            if burn_near_fire(x, y, read_state, write_state) == FireResult::Burnt {
                return;
            }

            if dissolve_in_acid(x, y, read_state, write_state) == AcidResult::Dissolved {
                return;
            }

            write_state.write_cell(Cell::Wood, x, y, false);
        },
        Cell::Fire{heat} => {
            if heat <= 0 {
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
                    update_if_on_boundary(x, y, write_state);
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
                        _ => {
                            write_state.write_cell(Cell::Air, x, y, true)
                        }
                    }
                },
                GravityResult::Falling => {}
            }
        },
        Cell::Vine{growth, grown} => {
            if burn_near_fire(x, y, read_state, write_state) == FireResult::Burnt {
                return;
            }

            if dissolve_in_acid(x, y, read_state, write_state) == AcidResult::Dissolved {
                return;
            }

            let g = growth == 0 || grown;
            write_state.write_cell(Cell::Vine{growth: growth, grown: g}, x, y, !grown);

            if growth <= 0 || grown {
                return;
            }           

            let (dx, mut dy) = random_dir(x, y);
            if dy > y {
                dy = y;
            }  
            match read_state.read_cell(dx, dy) {
                Cell::Air => {                  
                    write_state.write_cell(Cell::Vine{growth: growth - 1, grown: false}, dx, dy, true);
                    write_state.write_cell(Cell::Vine{growth: growth, grown: true}, x, y, true);
                },
                _ => {
                    // No room to grow
                    write_state.write_cell(Cell::Vine{growth: growth - 1, grown: false}, x, y, true);
                }
            }
        },
        Cell::Water{dx} => {
            if dissolve_in_acid(x, y, read_state, write_state) == AcidResult::Dissolved {
                return;
            } 

            match gravity(cell, x, y, read_state, write_state) {
                GravityResult::OnGround => {

                                       

                    // ---------- TEST ----------
                    match read_state.read_cell(x, y - 1) {
                        Cell::Sand => {
                            write_state.write_cell(Cell::Water{dx: 0}, x, y - 1, true);
                            write_state.write_cell(Cell::Sand, x, y, true);
                            return;
                        },
                        _ => {}
                    }
                    // ---------- TEST ----------


                    let mut sideways = dx + x;
                    if dx == 0 {
                        sideways = random_axis(x);
                    }
                    let delta = sideways - x;
                    let inverse_sideways =  x - delta;
                    if read_state.is_empty(sideways, y) && write_state.is_empty(sideways, y) {
                        write_state.mark_block_dirty(inverse_sideways, y);
                        write_state.write_cell(Cell::Air, x, y, false);
                        write_state.write_cell(Cell::Water{dx: delta}, sideways, y, true);
                    }
                    else {
                        write_state.write_cell(Cell::Water{dx: 0}, x, y, true);
                    }
                },
                _ => {}
            }
        },
        Cell::Acid{t} => {
            match gravity(Cell::Acid{t: 1}, x, y, read_state, write_state) {
                GravityResult::OnGround => {
                    if t > 0 {
                        write_state.write_cell(Cell::Air, x, y, false);
                    }
                    write_state.mark_block_dirty(x, y + 1);
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

#[derive(PartialEq, Eq)]
enum FireResult {
    Unaffected,
    Burnt
}

#[derive(PartialEq, Eq)]
enum AcidResult {
    Unaffected,
    Dissolved
}

fn update_if_on_boundary(x: i32, y: i32, write_state: &mut GameState) {
    if x % REGION_SIZE == 0 {
        write_state.mark_block_dirty(x - 1, y);
    }
    if y % REGION_SIZE == 0 {
        write_state.mark_block_dirty(x, y - 1);
    }
    if x % REGION_SIZE == REGION_SIZE -1 {
        write_state.mark_block_dirty(x + 1, y);
    }
    if y % REGION_SIZE == REGION_SIZE -1 {
        write_state.mark_block_dirty(x, y + 1);
    }
}

fn burn_near_fire(x: i32, y: i32, read_state: &GameState, write_state: &mut GameState) -> FireResult{
    let (dx, dy) = random_dir(x, y);
    match read_state.read_cell(dx, dy) {
        Cell::Fire{..} => {
                write_state.write_cell(Cell::Fire{heat: 30}, x, y, true);
                FireResult::Burnt
        },
        _ => {
            FireResult::Unaffected
        }
    }
}

fn dissolve_in_acid(x: i32, y: i32, read_state: &GameState, write_state: &mut GameState) -> AcidResult {
    let (dx, dy) = random_dir(x, y);
    match read_state.read_cell(dx, dy) {
        Cell::Acid{..}=> {
            write_state.write_cell(Cell::Air, dx, dy, false);
            AcidResult::Dissolved
        },
        _ => {
            AcidResult::Unaffected
        }
    }
}

fn gravity(cell: Cell, x: i32, y: i32, read_state: &GameState, write_state: &mut GameState) ->  GravityResult{
    let new_y = y + 1;
    let mut sideways = - 1;
    if rand::random() {
        sideways =  1;
    }
    let new_x = x + sideways;
    let height = (read_state.size as i32) - 1;
    if  new_y <= height && read_state.is_empty(x, new_y) /*&& write_state.is_empty(x, new_y)*/ {
        write_state.write_cell(cell, x, new_y, true);
        if x % REGION_SIZE == 0 || y % REGION_SIZE == 0 {
            write_state.mark_block_dirty(x - sideways, y - 1);
        }
        GravityResult::Falling
    }
    else if new_y <= height && new_x >= 0 && new_x <= height && read_state.is_empty(new_x, new_y) /*&& write_state.is_empty(new_x, new_y)*/ {
         write_state.write_cell(cell, new_x, new_y, true);  
         if x & REGION_SIZE == 0 || y % REGION_SIZE == 0 {
            write_state.mark_block_dirty(x - sideways, y - 1);
         }
         GravityResult::Falling
    }
    else {
        write_state.write_cell(cell, x, y, false);
        GravityResult::OnGround
    }
}