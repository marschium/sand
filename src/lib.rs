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
mod input;

use cells::{Cell, RadialSpawner};

pub fn start() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("sand", render::SCREEN_SIZE as u32, 580 as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();

    let hud = render::Hud::new(&texture_creator);

    let tex_a = texture_creator.create_texture_target(None, render::MAP_SIZE as u32, render::MAP_SIZE as u32).map_err(|x| x.to_string()).unwrap();
    let tex_b = texture_creator.create_texture_target(None, render::MAP_SIZE as u32, render::MAP_SIZE as u32).map_err(|x| x.to_string()).unwrap();

    let mut read_state = game::GameState::new(render::MAP_SIZE, tex_a);
    let mut write_state = game::GameState::new(render::MAP_SIZE,tex_b);

    let mut frame_start = Instant::now();

    let mut frames = 0u32;
    let mut frame_log_timer = Duration::from_secs(0);
    let mut update_times = Vec::new();
    let mut draw_times = Vec::new();

    // TODO move?
    let mut spawner = RadialSpawner::new(5, 5);
    'running: loop {
        frame_start = Instant::now();

        for event in event_pump.poll_iter() { 
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Delete), ..} => {
                    read_state.clear();
                    write_state.clear();
                }
                _ => {
                    input::update_spawner(event, &mut spawner);
                }
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
        canvas.copy(&read_state.get_tex(), None, Rect::new(0, 0, 512, 512)).unwrap();
        hud.draw(&mut canvas);

        draw_times.push(draw_time.elapsed().as_micros());      
        canvas.present();

        // SLEEP
        let mut frame_end = frame_start.elapsed();
        frames += 1;
        if(frame_end < Duration::from_millis(16)) {
            std::thread::sleep(Duration::from_millis(16) - frame_end);
        }

        // LOG
        frame_log_timer = frame_log_timer.add(frame_start.elapsed());
        // if frame_log_timer >= Duration::from_millis(1000) {
        //     println!("FPS: {}", frames);

        //     let mut sum : u128 =  update_times.iter().sum();
        //     let mut avg =  sum as f64 /  update_times.len() as f64;
        //     println!("Update: {}", avg);

        //     sum = draw_times.iter().sum();
        //     avg = sum as f64 / draw_times.len() as f64;
        //     println!("Draw: {}", avg);

        //     frames = 0;
        //     frame_log_timer = Duration::from_millis(0);
        //     update_times.clear();
        //     draw_times.clear();
        // }
    }
}