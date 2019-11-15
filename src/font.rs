use rusttype::{point, Font,Scale,PositionedGlyph};


pub struct Glyph
{
	pub offest_x:usize,
	pub offest_y:usize,
	pub data:Vec<(usize,usize,f32)>
}

pub struct GlyphMap
{
	size:usize,
	spacing:usize,
	font:Font<'static>,
	scale:Scale,
	glyphs:Vec<Glyph>
}

impl GlyphMap
{
	pub fn load(size:usize) -> GlyphMap
	{ 
		let font_data = include_bytes!("../ressources/Monaco.ttf");
		let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

		let mut gm = GlyphMap{
			size: size,
			spacing: size,
			font: font,
			scale:Scale::uniform(size as f32),
			glyphs:vec![]
		};

		let mut spacing_needed = 0;
		for i in 0..255u8
		{
			let glyph = gm.glyph_from_char(i);
			gm.glyphs.push(glyph.0);
			spacing_needed = std::cmp::max(spacing_needed,glyph.1);
		}
		for glyph in &mut gm.glyphs
		{
			glyph.offest_x = spacing_needed.wrapping_sub(glyph.offest_x) / 2;
		}
		gm.spacing = spacing_needed;

		return gm;
	}

	pub fn glyph_size(&self) -> (usize,usize)
	{
		(self.spacing,self.size)
	}

	pub fn get(&self,c:u8) -> &Glyph
	{
		return &self.glyphs[c as usize];
	}

	fn glyph_from_char(&self,c:u8) -> (Glyph,usize)
	{
		let glyphs = self.get_glyphs(c);

		let g = glyphs.first();

		if g.is_none() { return (Glyph{offest_x:0,offest_y:0,data:vec![]},0); }
		let g = g.unwrap();

		if g.pixel_bounding_box().is_none() { return (Glyph{offest_x:0,offest_y:0,data:vec![]},0); }
		let bb = g.pixel_bounding_box().unwrap();

		let glyphs_width = GlyphMap::get_glyph_width(&g);

		let mut returned_glyph = Glyph{
			offest_x:(glyphs_width as usize),
			offest_y:if bb.min.y < 0 { 0 } else { bb.min.y as usize},
			data:vec![]
		};
		// println!("{:?}",glyphs_width );

		g.draw(|x, y, v| { returned_glyph.data.push((x as usize,y as usize,v));});

		return (returned_glyph,glyphs_width);
	}

	fn get_glyphs(&self,c:u8) -> Vec<PositionedGlyph>
	{
		let mut text = String::new();
		text.push(c as char);

		let v_metrics = self.font.v_metrics(self.scale);
		return self.font.layout(&text, self.scale, point(0.0, 0.0 + v_metrics.ascent)).collect();
	}

	fn get_glyph_width(glyph:&rusttype::PositionedGlyph) -> usize
	{
		let min_x = match glyph.pixel_bounding_box(){
			Some(bb) => bb.min.x,
			None => 0
		};	

		let max_x = match glyph.pixel_bounding_box(){
			Some(bb) => bb.max.x,
			None => 0
		};

		return max_x.wrapping_sub(min_x) as usize;
	}
}