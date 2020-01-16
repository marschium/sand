use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};
use crate::cells::Cell;

const TEX_SIZE: u32 = 16;

pub fn get_cell_color(cell: Cell) -> Color {
    match cell {
        Cell::Sand => {
            Color::RGB(180, 155, 3)
        },
        Cell::Wood{..} => {
            Color::RGB(116, 43, 0)
        },
        Cell::Fire{..} => {
            Color::RGB(255, 0, 0)
        },
        Cell::Seed | Cell::Vine{..} => {
            Color::RGB(0, 116, 11)
        }
        _ => Color::RGB(0, 0, 0)
    }
}
