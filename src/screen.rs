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
		let font = font::GlyphMap::load_png("ressources/font.png");
		let glyph_size = font.glyph_size();
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
		let glyph_size = self.font.glyph_size();
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
		for p in glyph
		{
			let pos:usize   = (p.1+pix_y)*self.width+(p.0+pix_x);
			let pixel       = self.font.pixel_from_id(p.2).unwrap();
			self.buffer[pos] = if pixel { fg } else { bg };
		}
	}

	pub fn buffer(&self) -> &Vec<u32>
	{
		return &self.buffer;
	}

	pub fn real_width(&self)  -> usize { self.width  }
	pub fn real_height(&self) -> usize { self.height }
}

