extern crate minifb;
extern crate rusttype;
use std::{thread, time};
use minifb::{Key, WindowOptions, Window};

mod font;
mod color;
mod screen;
mod view;

const WIDTH: usize = 50;
const HEIGHT: usize = 30;
const TITLE: &str = "TUI Example - ESC to exit";

fn main() 
{

	let mut screen = screen::Screen::new(WIDTH,HEIGHT,color::BLUE);
	screen.draw_at(&String::from("0123456789,ABCEDFGHIJKLMNOPQRSTUVWXYZ\n"),1,1,color::RED,color::BLUE);

	let mut view  = view::View::new(10,10,10,10);
	view.draw_at(String::from("test"),3,3);

	let window = Window::new(TITLE,screen.real_width(),screen.real_height(),WindowOptions::default());
	let mut window = window.unwrap_or_else(|e| {panic!("{}", e);});

	let sixiteen_millis = time::Duration::from_millis(16);
	while window.is_open() && !window.is_key_down(Key::Escape)
	{
		let start = time::Instant::now();
		view.apply(&mut screen);
		// do stuff here
		let delta = time::Instant::now() - start;
		if sixiteen_millis > delta {
			thread::sleep(sixiteen_millis-(delta));
		}
		window.update_with_buffer(screen.buffer()).unwrap();
	}
}