use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::cells::{Cell, RadialSpawner};
use crate::render;


pub fn update_spawner(event: Event, spawner: &mut RadialSpawner) {
    match event {
        Event::MouseMotion{x, y, ..} => {
            spawner.set_pos((x as f32 * render::MOUSE_RATIO) as i32, (y as f32 * render::MOUSE_RATIO) as i32);
        }
        Event::MouseButtonDown{..} => {
            spawner.enable();
        },
        Event::MouseButtonUp{..} => {
            spawner.disable();
        },
        Event::KeyDown {keycode: Some(Keycode::Q), ..} => {
            spawner.set_cell(Cell::Wood);
        },
        Event::KeyDown {keycode: Some(Keycode::W), ..} => {
            spawner.set_cell(Cell::Fire{heat: 30});
        },
        Event::KeyDown {keycode: Some(Keycode::E), ..} => {
            spawner.set_cell(Cell::Seed);
        },
        Event::KeyDown {keycode: Some(Keycode::R), ..} => {
            spawner.set_cell(Cell::Water{dx: 0});
        },
        Event::KeyDown {keycode: Some(Keycode::T), ..} => {
            spawner.set_cell(Cell::Acid{t: 0});
        },
        Event::KeyDown {keycode: Some(Keycode::Y), ..} => {
            spawner.set_cell(Cell::Sand);
        },
        Event::KeyDown {keycode: Some(Keycode::U), ..} => {
            spawner.set_cell(Cell::Rocket{last_pos: (-1, -1), i: -1});
        },
        Event::KeyDown {keycode: Some(Keycode::I), ..} => {
            spawner.set_cell(Cell::Stone);
        },
        Event::KeyDown {keycode: Some(Keycode::O), ..} => {
            spawner.set_cell(Cell::Bomb);
        },
        Event::KeyDown {keycode: Some(Keycode::P), ..} => {
            spawner.set_cell(Cell::Ice);
        },
        _ => {}
    }
}


pub fn get_key_bindings() -> Vec<(String, String)> {
    vec!(
        ("Q".to_owned(), "Wood".to_owned()),
        ("W".to_owned(), "Fire".to_owned()),
        ("E".to_owned(), "Seed".to_owned()),
        ("R".to_owned(), "Water".to_owned()),
        ("T".to_owned(), "Acid".to_owned()),
        ("Y".to_owned(), "Sand".to_owned()),
        ("U".to_owned(), "Rocket".to_owned()),
        ("I".to_owned(), "Stone".to_owned()),
        ("O".to_owned(), "Bomb".to_owned()),
        ("P".to_owned(), "Ice".to_owned()),
        ("LMB".to_owned(), "Spawn".to_owned()),
        ("DEL".to_owned(), "Clear".to_owned()),
        ("ESC".to_owned(), "Exit".to_owned())
    )
}