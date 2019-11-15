use font;
use color;

pub struct Screen{
	width:usize,
	height:usize,
	buffer:Vec<u32>,
	font:font::GlyphMap
}

impl Screen
{
	pub fn new(width:usize,height:usize,bg:color::Color) -> Screen
	{
		let font = font::GlyphMap::load(20,20);
		let glyph_size = font.glyph_size().0;
		let real_width  = width  * glyph_size;
		let real_height = height * glyph_size;
		return Screen{
			width  : real_width,
			height : real_height,
			buffer : vec![bg;real_width*real_height],
			font   : font
		};
	}

	pub fn draw_at(&mut self, string:&String,x:usize,y:usize,fg:color::Color,bg:color::Color)
	{
		let glyph_size = self.font.glyph_size().0;
		let mut pix_x = x * glyph_size;
		let     pix_y = y * glyph_size;

		for c in string.bytes()
		{
			let glyph = self.font.glyph_from_char(c);
			self.draw_glyph_at(glyph,pix_x,pix_y,fg,bg);
			pix_x += glyph_size;
		}
	}

	fn draw_glyph_at(&mut self, glyph:font::Glyph, pix_x:usize,pix_y:usize,fg:color::Color,bg:color::Color)
	{
		for j in 0..self.font.glyph_size().0
		{
			for i in 0..self.font.glyph_size().0
			{
				let pos:usize    = (j+pix_y)*self.width+(i+pix_x);
				self.buffer[pos] =  bg;
			}
		}
		for p in glyph.data.iter()
		{
			let pos:usize   = (p.1+pix_y+glyph.offest_y)*self.width+(p.0+pix_x+glyph.offest_x);
			self.buffer[pos] =  color::mix(fg,bg,p.2);
		}
	}

	pub fn buffer(&self) -> &Vec<u32>
	{
		return &self.buffer;
	}

	pub fn real_width(&self)  -> usize { self.width  }
	pub fn real_height(&self) -> usize { self.height }
}

