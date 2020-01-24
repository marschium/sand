use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};
use crate::cells::Cell;
use crate::input;

pub const SCREEN_SIZE: i32 = 512;
pub const MAP_SIZE: i32 = 256; // TODO move somewhere sensible
pub const MOUSE_RATIO: f32 = MAP_SIZE as f32 / SCREEN_SIZE as f32;

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
        },
        Cell::Water{..} => {
            Color::RGB(16, 16, 116)
        },
        Cell::Acid{..} => {
            Color::RGB(16, 116, 16)
        },
        Cell::Rocket{..} => {
            Color::RGB(255, 255, 255)
        },
        Cell::Stone => {
            Color::RGB(116, 116, 116)
        },
        Cell::Bomb => {
            Color::RGB(116, 116, 16)
        }
        _ => Color::RGB(0, 0, 0)
    }
}


pub struct Hud<'a> {
    keybinding_textures: Vec<Texture<'a>>,
}

impl<'a> Hud<'a> { 
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Hud {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let mut font = ttf_context.load_font("Jazz_Ball_Regular.ttf", 32).unwrap();
        
        let mut keybinding_textures = Vec::new(); 
        for (key, element) in input::get_key_bindings() {
            let sur = font.render(&format!("({})  {}", key, element)).blended(Color::RGBA(255, 255, 255, 255)).unwrap();
            keybinding_textures.push(texture_creator.create_texture_from_surface(&sur).unwrap());
        }

        Hud {
            keybinding_textures
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        // TODO track position and size of hud
        let mut row = 0;
        let mut col = 0;
        let row_off = 16;
        let col_off = 128;
        for tex in self.keybinding_textures.iter() {
            canvas.copy(&tex, None, Rect::new(0 + (col_off * col), 512 + (row_off * row), 64, 16)).unwrap();
            row += 1;
            if row >= 4 {
                row = 0;
                col += 1;
            }
        }
    }
}