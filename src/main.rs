extern crate minifb;
use std::{thread, time};
use minifb::{Key, WindowOptions, Window};

mod font;
mod color;
mod screen;

const WIDTH: usize = 100;
const HEIGHT: usize = 60;
const TITLE: &str = "TUI Example - ESC to exit";

fn main() 
{
	let mut screen = screen::Screen::new(WIDTH,HEIGHT,color::BLUE);
	screen.draw_at("0123456789\nABCEDFGHIJKLMNOPQRSTUVWXYZ\n",1,1,color::RED,color::BLUE);

	let window = Window::new(TITLE,screen.real_width(),screen.real_height(),WindowOptions::default());
	let mut window = window.unwrap_or_else(|e| {panic!("{}", e);});

	let sixiteen_millis = time::Duration::from_millis(16);
	while window.is_open() && !window.is_key_down(Key::Escape)
	{
		let start = time::Instant::now();
		// do stuff here
		let delta = time::Instant::now() - start;
		if sixiteen_millis > delta {
			thread::sleep(sixiteen_millis-(delta));
		}
		window.update_with_buffer(screen.buffer()).unwrap();
	}
}