extern crate minifb;
extern crate r_tui;
use std::{thread, time};
use minifb::{Key, WindowOptions, Window};

use r_tui::color_table;
use r_tui::screen;
use r_tui::view;

const WIDTH: usize = 50;
const HEIGHT: usize = 30;
const FONT_SIZE: usize = 16;
const TITLE: &str = "TUI Example - ESC to exit";

fn main() 
{

	let mut screen = screen::Screen::new(WIDTH,HEIGHT,FONT_SIZE,color_table::BLUE);
	screen.draw_at(&String::from("AZERTYUIOPQSDFGHJKLMWXCVBN"),1,1,color_table::RED,color_table::BLACK);

	let mut view  = view::View::new(10,10,10,10);
	view.draw_at(String::from("testtesttest\ntesttesttest\ntesttesttest\ntesttesttest\ntesttesttest\ntesttesttest\ntesttesttest\ntesttesttest\ntesttesttest\ntesttesttest\ntesttesttest\ntesttesttest\ntesttesttest\n"),0,0);

	let mut view2  = view::View::new(22,5,10,10);
	view2.stream(String::from("testtesttest\ntesttest"));
	view2.color(color_table::ELECTRIC_INDIGO,color_table::DEEP_PINK);
	view2.stream(String::from("test\ntesttesttest\nt"));
	view2.stream(String::from("esttest"));
	view2.color(color_table::RED,color_table::OLIVE);
	view2.stream(String::from("test\ntesttesttesttesttesttest"));

	let window = Window::new(TITLE,screen.real_width(),screen.real_height(),WindowOptions::default());
	let mut window = window.unwrap_or_else(|e| {panic!("{}", e);});

	let sixiteen_millis = time::Duration::from_millis(16);
	while window.is_open() && !window.is_key_down(Key::Escape)
	{
		let start = time::Instant::now();
		view.apply(&mut screen);
		view2.apply(&mut screen);
		// do stuff here
		let delta = time::Instant::now() - start;
		if sixiteen_millis > delta {
			thread::sleep(sixiteen_millis-(delta));
		}
		if screen.is_dirty()
		{
			window.update_with_buffer(screen.buffer()).unwrap();
		}
		else{
			window.update();
		}
	}
}