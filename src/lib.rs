extern crate sdl2;
use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

mod game;
mod cells;
mod render;

use cells::{Cell, RadialSpawner};

const CELL_DRAW_SIZE: i32 = 2;
const MAP_SIZE: i32 = 16;

pub fn start() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("sand", 512, 512)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    // TODO lookup?
    let sand_tex = render::create_cell_texture(&Color::RGB(255, 255, 102), &mut canvas, &texture_creator).unwrap();
    let wood_tex = render::create_cell_texture(&Color::RGB(111, 48, 15), &mut canvas, &texture_creator).unwrap();   
    let fire_tex = render::create_cell_texture(&Color::RGB(255, 16, 16), &mut canvas, &texture_creator).unwrap();      
    let plant_tex = render::create_cell_texture(&Color::RGB(16, 255, 16), &mut canvas, &texture_creator).unwrap();

    let mut read_state = game::GameState::new(MAP_SIZE);
    let mut write_state = game::GameState::new(MAP_SIZE);

    let mut frame_start = Instant::now();
    let mut ms_since_update = 0u128;

    let mut frames_since_log = 0u32;
    let mut frame_times = Vec::new();

    // TODO move?
    let mut mouse_down = false;
    let mut mouse_pos = (0i32, 0i32);
    let mut spawner = RadialSpawner::new(5, 5);

    'running: loop {
        frame_start = Instant::now();
        // TODO get constant framerate

        for event in event_pump.poll_iter() { 
            match event {
                Event::MouseMotion{x, y, ..} => {
                    spawner.set_pos(x / CELL_DRAW_SIZE, y / CELL_DRAW_SIZE);
                }
                Event::MouseButtonDown{..} => {
                    spawner.enable();
                },
                Event::MouseButtonUp{..} => {
                    spawner.disable();
                },
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Q), ..} => {
                    spawner.set_cell(Cell::Wood{fuel: 5});
                },
                Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    spawner.set_cell(Cell::Fire{heat: 30});
                },
                Event::KeyDown {keycode: Some(Keycode::E), ..} => {
                    spawner.set_cell(Cell::Seed);
                }
                _ => {}
            }
        }

        if ms_since_update >= 16 { // 60 updates per sec
            frames_since_log += 1;
            ms_since_update = 0;
            let update_start = Instant::now();
            game::update(&read_state, &mut write_state, &mut spawner);
            frame_times.push(update_start.elapsed().as_micros());
            let tmp = read_state;
            read_state = write_state;
            write_state = tmp;
        }


        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for (pos, b) in read_state.blocks.iter() {  // todo include offset in block
            let block_offset = (pos.0 * game::REGION_SIZE, pos.1 * game::REGION_SIZE);
            for (c, i, j) in b.cells() {
                //let tex = texture_lookup.entry(c.clone()).or_insert_with(|| { render::create_cell_texture(&Color::RGB(255, 255, 102), &mut canvas, &texture_creator).unwrap()});
                match c {
                    Cell::Air => {},
                    // TODO lookup for something
                    Cell::Sand => {
                        canvas.copy(
                            &sand_tex,
                            None,
                            Rect::new(
                                (block_offset.0 + i) * CELL_DRAW_SIZE,
                                (block_offset.1 + j) * CELL_DRAW_SIZE,
                                CELL_DRAW_SIZE as u32, CELL_DRAW_SIZE as u32))
                            .unwrap();
                    },
                    Cell::Wood{..} => {
                        canvas.copy(
                            &wood_tex,
                            None,
                            Rect::new(
                                (block_offset.0 + i) * CELL_DRAW_SIZE,
                                (block_offset.1 + j) * CELL_DRAW_SIZE,
                                CELL_DRAW_SIZE as u32, CELL_DRAW_SIZE as u32))
                            .unwrap();
                    },
                    Cell::Fire{..} => {
                        canvas.copy(
                            &fire_tex,
                            None,
                            Rect::new(
                                (block_offset.0 + i) * CELL_DRAW_SIZE,
                                (block_offset.1 + j) * CELL_DRAW_SIZE,
                                CELL_DRAW_SIZE as u32, CELL_DRAW_SIZE as u32))
                            .unwrap();
                    },
                    Cell::Seed | Cell::Vine{..} => {
                        canvas.copy(
                            &plant_tex,
                            None,
                            Rect::new(
                                (block_offset.0 + i) * CELL_DRAW_SIZE,
                                (block_offset.1 + j) * CELL_DRAW_SIZE,
                                CELL_DRAW_SIZE as u32, CELL_DRAW_SIZE as u32))
                            .unwrap();
                    }
                }                
            }
        }
        

        canvas.present();

        let mut d = frame_start.elapsed().as_millis();
        if d == 0 {
            d = 1;
        }
        ms_since_update += d;

        if frames_since_log >= 60 {
            let sum : u128 =  frame_times.iter().sum();
            let avg =  sum as f64 /  frame_times.len() as f64;
            println!("{}", avg);
            frames_since_log = 0;
            frame_times.clear();
        }
    }
}