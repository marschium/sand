use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};
use crate::cells::Cell;

const TEX_SIZE: u32 = 16;

pub fn create_cell_texture<'a>(color: &Color, canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Texture<'a>,String> {
    let mut tex = texture_creator.create_texture_target(None, TEX_SIZE, TEX_SIZE).map_err(|x| x.to_string())?;
    canvas.with_texture_canvas(&mut tex, |c| {
        for i in 0..TEX_SIZE {
            for j in 0..TEX_SIZE {
                if i == 0 || j == 0 || i == TEX_SIZE - 1 || j == TEX_SIZE - 1 {
                    c.set_draw_color(Color::RGB(255, 0, 0));
                }
                else{
                    c.set_draw_color(color.clone());
                }
                c.draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
            }
        }
    }).map_err(|x| x.to_string())?;
    Ok(tex)
}
