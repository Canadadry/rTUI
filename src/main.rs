extern crate minifb;
use std::{thread, time};
use minifb::{Key, WindowOptions, Window};

mod font;
mod view;

const WIDTH: usize = 100*font::CHAR_WIDTH;
const HEIGHT: usize = 66*font::CHAR_HEIGHT;
const TITLE: &str = "TUI Example - ESC to exit";

fn main() 
{
	let window = Window::new(TITLE,WIDTH,HEIGHT,WindowOptions::default());
	let mut window = window.unwrap_or_else(|e| {panic!("{}", e);});

	let mut screen = view::Screen::new(WIDTH,HEIGHT);
	screen.draw_at("0123456789\nABCEDFGHIJKLMNOPQRSTUVWXYZ\n",0,0);

	let sixiteen_millis = time::Duration::from_millis(16);
	while window.is_open() && !window.is_key_down(Key::Escape)
	{
		let start = time::Instant::now();
		// do stuff here
		let delta = time::Instant::now() - start;
		if sixiteen_millis > delta {
			thread::sleep(sixiteen_millis-(delta));
		}

		window.update_with_buffer(screen.get_buffer()).unwrap();
	}
}