use rusttype::{point, Font,Scale,PositionedGlyph};

pub struct GlyphMapBuilder
{
	font:Font<'static>,
	scale:Scale	
}

impl GlyphMapBuilder
{

	pub fn load(size:usize) -> GlyphMapBuilder
	{ 
		let font_data = include_bytes!("../ressources/Monaco.ttf");

		return GlyphMapBuilder{
			font  : Font::from_bytes(font_data as &[u8]).expect("Error constructing Font"),
			scale : Scale::uniform(size as f32)
		};
	}

	pub fn glyph_from_char(&self,c:u8) -> (usize,usize,Vec<(usize,usize,f32)>)
	{
		let glyphs = self.get_glyphs(c);

		let g = glyphs.first();

		if g.is_none() { return (0,0,vec![]); }
		let g = g.unwrap();

		if g.pixel_bounding_box().is_none() { return (0,0,vec![]); }
		let bb = g.pixel_bounding_box().unwrap();

		let glyphs_width = GlyphMapBuilder::get_glyph_width(&g);

		let mut returned_glyph = (
			glyphs_width as usize,
			if bb.min.y < 0 { 0 } else { bb.min.y as usize},
			vec![]
		);

		g.draw(|x, y, v| { (returned_glyph.2).push((x as usize,y as usize,v));});

		return returned_glyph;
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