use font;
use color;
use sub_screen;


pub struct Screen{
	width:usize,
	height:usize,
	buffer:Vec<u32>,
	font:font::GlyphMap,
	dirty:bool
}

impl Screen
{
	pub fn new(width:usize,height:usize,font_size:usize,bg:color::Color) -> Screen
	{
		let font = font::GlyphMap::load(font_size);
		let glyph_size = font.glyph_size();
		let real_width  = width  * glyph_size.0;
		let real_height = height * glyph_size.1;
		return Screen{
			width  : real_width,
			height : real_height,
			buffer : vec![bg;real_width*real_height],
			font   : font,
			dirty  : true,
		};
	}

	pub fn draw_at(&mut self, string:&String,x:usize,y:usize,fg:color::Color,bg:color::Color)
	{
		let glyph_size = self.font.glyph_size();
		let mut pix_x = x * glyph_size.0;
		let     pix_y = y * glyph_size.1;

		for c in string.bytes()
		{
			self.draw_char_at(c,pix_x,pix_y,fg,bg);
			pix_x += glyph_size.0;
		}
		self.dirty = true;
	}

	pub fn sub(&self, left:usize,width:usize,top:usize,height:usize) -> Option<sub_screen::SubScreen>
	{
		return sub_screen::SubScreen::from(left,width,top,height,&self);
	}

	fn draw_char_at(&mut self, c:u8, pix_x:usize,pix_y:usize,fg:color::Color,bg:color::Color)
	{
		let glyph = self.font.get(c);
		for j in 0..self.font.glyph_size().1
		{
			for i in 0..self.font.glyph_size().0
			{
				let pos:usize    = (j+pix_y)*self.width+(i+pix_x);
				self.buffer[pos] =  bg;
			}
		}
		for p in glyph.data.iter()
		{
			let pos:usize   = (p.1+pix_y+glyph.offset_y)*self.width+(p.0+pix_x+glyph.offset_x);
			self.buffer[pos] =  color::mix(fg,bg,p.2);
		}
	}

	pub fn is_dirty(&self) -> bool
	{
		return self.dirty;
	}

	pub fn buffer(&mut self) -> &Vec<u32>
	{
		self.dirty = false;
		return &self.buffer;
	}

	pub fn buffer_mut(&mut self) -> &mut Vec<u32>
	{
		self.dirty = true;
		return &mut self.buffer;
	}

	pub fn real_width(&self)  -> usize { self.width  }
	pub fn real_height(&self) -> usize { self.height }
}

