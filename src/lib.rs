extern crate sdl2;
use std::ops::Add;
use std::collections::HashMap;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Instant, Duration};

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

    let tex_a = texture_creator.create_texture_target(None, 256, 256).map_err(|x| x.to_string()).unwrap();
    let tex_b = texture_creator.create_texture_target(None, 256, 256).map_err(|x| x.to_string()).unwrap();

    let mut read_state = game::GameState::new(MAP_SIZE, tex_a);
    let mut write_state = game::GameState::new(MAP_SIZE,tex_b);

    let mut frame_start = Instant::now();

    let mut frames = 0u32;
    let mut frame_log_timer = Duration::from_secs(0);
    let mut update_times = Vec::new();
    let mut draw_times = Vec::new();

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
                },
                Event::KeyDown {keycode: Some(Keycode::R), ..} => {
                    spawner.set_cell(Cell::Water{dx: 0});
                }
                _ => {}
            }
        }

        // UPDATE
        let update_start = Instant::now();

        canvas.with_texture_canvas(write_state.get_tex(), |c| {
            c.copy(read_state.get_tex(), None, None);
            });

        game::update(&read_state, &mut write_state, &mut spawner);
        update_times.push(update_start.elapsed().as_micros());

        let tmp = read_state;
        read_state = write_state;
        write_state = tmp;

        // DRAW
        let draw_time = Instant::now();
        canvas.copy(&read_state.get_tex(), None, None).unwrap();
        draw_times.push(draw_time.elapsed().as_micros());      
        canvas.present();

        // SLEEP
        let mut frame_end = frame_start.elapsed();
        frames += 1;
        if(frame_end < Duration::from_millis(16)) {
            std::thread::sleep(Duration::from_millis(16) - frame_end);
        }
        frame_log_timer = frame_log_timer.add(frame_start.elapsed());

        if frame_log_timer >= Duration::from_millis(1000) {
            println!("FPS: {}", frames);

            let mut sum : u128 =  update_times.iter().sum();
            let mut avg =  sum as f64 /  update_times.len() as f64;
            println!("Update: {}", avg);

            sum = draw_times.iter().sum();
            avg = sum as f64 / draw_times.len() as f64;
            println!("Draw: {}", avg);

            frames = 0;
            frame_log_timer = Duration::from_millis(0);
            update_times.clear();
            draw_times.clear();
        }
    }
}