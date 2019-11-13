extern crate minifb;
use minifb::{Key, KeyRepeat, WindowOptions, Window};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const TITLE: &str = "TUI Example - ESC to exit";

fn main() {
    let window = Window::new(TITLE,WIDTH,HEIGHT,WindowOptions::default());
    let mut window = window.unwrap_or_else(|e| {panic!("{}", e);});

    let buffer =[ 0u32 ; WIDTH*HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape)
    {
        window.update_with_buffer(&buffer).unwrap();
    }
}