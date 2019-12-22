extern crate sdl2;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};

const TEX_SIZE: u32 = 16;

pub fn dummy_texture<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Texture<'a>,String> {
    let mut tex = texture_creator.create_texture_target(None, TEX_SIZE, TEX_SIZE).map_err(|x| x.to_string())?;
    canvas.with_texture_canvas(&mut tex, |c| {
        for i in 0..TEX_SIZE {
            for j in 0..TEX_SIZE {
                if (i + j) % 9 == 0 {
                    c.set_draw_color(Color::RGB(255, 255, 0));
                }
                else{
                    c.set_draw_color(Color::RGB(0, 0, 0));
                }
                c.draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
            }
        }
    }).map_err(|x| x.to_string())?;
    Ok(tex)
}

const GRID_SIZE: i32 = 32;

pub fn start() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("sand", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // TODO create a texture that acts as each sqaure
    let texture_creator = canvas.texture_creator();
    let tex = dummy_texture(&mut canvas, &texture_creator).unwrap();

    'running: loop {
        for event in event_pump.poll_iter() { 
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }


        canvas.clear();

        // for each cell in game draw the appropiate texture
        //canvas.copy(&tex, None, Rect::new(0, 0, TEX_SIZE, TEX_SIZE)).unwrap();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                canvas.copy(
                    &tex,
                    None,
                    Rect::new(i * TEX_SIZE as i32, j * TEX_SIZE as i32, TEX_SIZE, TEX_SIZE)).unwrap();
            }
        }

        canvas.present();
    }
}