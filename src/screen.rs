use font;
use color;

pub struct Screen{
	width:usize,
	height:usize,
	buffer:Vec<u32>,
	font:font::Font
}

impl Screen
{
	pub fn new(width:usize,height:usize,bg:color::Color) -> Screen
	{
		let real_width  = width  * (font::CHAR_WIDTH  + font::CHAR_SPACE);
		let real_height = height * (font::CHAR_HEIGHT + font::LINE_SPACE);
		Screen{
			width  : real_width,
			height : real_height,
			buffer : vec![bg;real_width*real_height],
			font   : font::get()
		}
	}

	pub fn draw_at(&mut self, string:&str,x:usize,y:usize,fg:color::Color,bg:color::Color)
	{
		let mut x_mut = x * (font::CHAR_WIDTH  + font::CHAR_SPACE);
		let mut y_mut = y * (font::CHAR_HEIGHT + font::LINE_SPACE);
		for c in string.bytes()
		{
			if c == b'\n'
			{
				y_mut += font::CHAR_HEIGHT + font::LINE_SPACE;
				x_mut = x * (font::CHAR_WIDTH  + font::CHAR_SPACE);
				continue;
			}
			let glyph = self.font[c as usize];
			for j in 0..font::CHAR_HEIGHT
			{
				for i in 0..font::CHAR_WIDTH
				{
					if x_mut+i >=self.width  {continue};
					if y_mut+j >=self.height {continue};
					let pos   = (y_mut+j)*self.width + (x_mut+i);
					let mask  = 1u8<<(font::CHAR_WIDTH-i-1);
					let byte  = glyph[j];
					let pixel = byte & mask == mask;
					self.buffer[pos] = if pixel { fg } else { bg };
				}			

			}
			for j in 0..font::LINE_SPACE
			{
				let pos   = (y_mut+font::CHAR_HEIGHT+j)*self.width + (x_mut);
				for i in 0..(font::CHAR_WIDTH + font::CHAR_SPACE)
				{
					self.buffer[pos+i] = bg;
				}
			}
			x_mut += font::CHAR_WIDTH + font::CHAR_SPACE;
		}
	}

	pub fn buffer(&self) -> &Vec<u32>
	{
		return &self.buffer;
	}

	pub fn real_width(&self)  -> usize { self.width  }
	pub fn real_height(&self) -> usize { self.height }
}

