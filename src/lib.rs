extern crate sdl2;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};
use std::time::{Instant, Duration};

mod game;

const TEX_SIZE: u32 = 16;

pub fn dummy_texture<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Texture<'a>,String> {
    let mut tex = texture_creator.create_texture_target(None, TEX_SIZE, TEX_SIZE).map_err(|x| x.to_string())?;
    canvas.with_texture_canvas(&mut tex, |c| {
        for i in 0..TEX_SIZE {
            for j in 0..TEX_SIZE {
                if i == 0 || j == 0 || i == TEX_SIZE - 1 || j == TEX_SIZE - 1 {
                    c.set_draw_color(Color::RGB(255, 0, 0));
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

const GRID_SIZE: i32 = 8;
const CELL_DRAW_SIZE: u32 = 32;

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

    let mut read_state = game::GameState::new();
    let mut write_state = game::GameState::new();
    for i in 0..game::REGION_SIZE {
        read_state.write_cell(game::Cell::Sand{delta: 1}, i , 0);
    }

    let mut frame_start = Instant::now();
    let mut ms_since_update = 0u128;

    'running: loop {
        frame_start = Instant::now();
        // TODO get constant framerate

        for event in event_pump.poll_iter() { 
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }

        // Update 60 times a second
        if ms_since_update >= 16 {
            ms_since_update = 0;
            read_state.update(&mut write_state);
            let tmp = read_state;
            read_state = write_state;
            write_state = tmp;
        }


        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for b in read_state.blocks.iter() {  // todo include offset in block
            for (c, i, j) in b.cells() {
                match c {
                    game::Cell::Sand{..} => {
                        canvas.copy(
                            &tex,
                            None,
                            Rect::new((i * CELL_DRAW_SIZE) as i32, (j * CELL_DRAW_SIZE) as i32, CELL_DRAW_SIZE, CELL_DRAW_SIZE)).unwrap();
                    },
                    _ => {}
                }
            }
        }
        

        canvas.present();

        ms_since_update += frame_start.elapsed().as_millis();
    }
}