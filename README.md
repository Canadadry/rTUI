# rTUI 

An attempt to create a simple tool for build Textual User Interface outside of the terminal. 

This is an exec for now but will be a lib (not related to minifb).

*Why ?* 

The main purpose for this is to build debugging tool and utility developpement. 
I want them outside of the terminal to allow me to and more rich feature in a near futur like image or event video. 

# Example

Here a basic example : 

```rust
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
		if screen.is_dirty()
		{
			window.update_with_buffer(screen.buffer()).unwrap();
		}
		else{
			window.update();
		}
	}
}
```

![screen 0](screenshot/screen_0.png)

To run it just do :
```bash
cd example; cargo run
```

Later I'll Add a more intersting example. Rigth now there is not much more to say than this screenshot show. 


# Usage 

There is 3 main struct : 

 * `Screen` : hold the pixel buffer and expose a basic draw at function. 
 * `View` : allow to split the screen into several area. Also have more advande drawing function
 * `color_table` : not a struct, but a module wich list basic color to start width. 

## Color table

* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/000000.png) : `color_table::BLACK`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/000080.png) : `color_table::NAVY`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/0000FF.png) : `color_table::BLUE`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/008000.png) : `color_table::GREEN`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/008080.png) : `color_table::TEAL`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/0080FF.png) : `color_table::DODGEBLUE`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/00FF00.png) : `color_table::LIME`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/00FF80.png) : `color_table::SPRING`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/00FFFF.png) : `color_table::AQUA`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/800000.png) : `color_table::MAROON`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/800080.png) : `color_table::PURPLE`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/8000FF.png) : `color_table::ELECTRIC_INDIGO`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/808000.png) : `color_table::OLIVE`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/808080.png) : `color_table::GRAY`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/8080FF.png) : `color_table::LIGHT_SLATE_BLUE`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/80FF00.png) : `color_table::CHARTREUSE`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/80FF80.png) : `color_table::LIGHT_GREEN`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/80FFFF.png) : `color_table::ELECTRIC_BLUE`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/FF0000.png) : `color_table::RED`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/FF0080.png) : `color_table::DEEP_PINK`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/FF00FF.png) : `color_table::FUCHSIA`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/FF8000.png) : `color_table::ORANGE`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/FF8080.png) : `color_table::LIGHT_CORAL`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/FF80FF.png) : `color_table::FUSHIA_PINK`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/FFFF00.png) : `color_table::YELLOW`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/FFFF80.png) : `color_table::WITCH_HAZE`
* ![screen 0](https://www.htmlcsscolor.com/preview/32x32/ffffff.png) : `color_table::WHITE`


