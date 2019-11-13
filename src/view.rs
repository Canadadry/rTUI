use font;

mod color{
	pub const WHITE:u32 = 0xffffff;
	pub const BLACK:u32 = 0x000000;
}

pub struct Screen{
	width:usize,
	height:usize,
	buffer:Vec<u32>,
	font:font::Font
}

impl Screen
{
	pub fn new(width:usize,height:usize) -> Screen
	{
		Screen{
			width:width,
			height:height,
			buffer: vec![0;width*height],
			font:font::get()
		}
	}

	pub fn draw_at(&mut self, string:&str,x:usize,y:usize)
	{
		let mut x_mut = x;
		let mut y_mut = y;
		for c in string.bytes()
		{
			if c == b'\n'
			{
				y_mut += font::CHAR_HEIGHT + font::LINE_SPACE;
				x_mut = x;
				continue;
			}
			let glyph = self.font[c as usize];
			for i in 0..font::CHAR_WIDTH
			{
				for j in 0..font::CHAR_HEIGHT
				{
					if x_mut+i >=self.width  {continue};
					if y_mut+j >=self.height {continue};
					let pos   = (y_mut+j)*self.width + (x_mut+i);
					let mask  = 1u8<<(font::CHAR_WIDTH-i-1);
					let byte  = glyph[j];
					let pixel = byte & mask == mask;
					self.buffer[pos] = if pixel { color::WHITE } else { color::BLACK };
				}		
			}
			x_mut += font::CHAR_WIDTH + font::CHAR_SPACE;
		}
	}

	pub fn get_buffer(&self) -> &Vec<u32>
	{
		return &self.buffer;
	}
}

