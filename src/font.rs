use font_builder;

pub struct Glyph
{
	pub offset_x:usize,
	pub offset_y:usize,
	pub data:Vec<(usize,usize,f32)>
}

pub struct GlyphMap
{
	width:usize,
	height:usize,
	glyphs:Vec<Glyph>
}

impl GlyphMap
{

	pub fn load(height:usize) -> GlyphMap
	{
		return GlyphMap::load_with_spacing(height,2);
	}

	pub fn load_with_spacing(height:usize,spacing:usize) -> GlyphMap
	{ 
		let mut gm = GlyphMap{
			width: height,
			height: height,
			glyphs:vec![]
		};

		gm.build_glyphs(spacing);

		return gm;
	}

	pub fn glyph_size(&self) -> (usize,usize)
	{
		(self.width,self.height)
	}

	pub fn get(&self,c:u8) -> &Glyph
	{
		return &self.glyphs[c as usize];
	}

	fn build_glyphs(&mut self,spacing:usize)
	{
		let builder = font_builder::GlyphMapBuilder::load(self.height);

		let mut max_width = 0;
		for i in 0..255u8
		{
			let glyph = builder.glyph_from_char(i);
			self.glyphs.push(Glyph{
				offset_x:glyph.0,
				offset_y:glyph.1,
				data:glyph.2});
			max_width = std::cmp::max(max_width,glyph.0);
		}
		max_width += spacing;
		for glyph in &mut self.glyphs
		{
			glyph.offset_x = max_width.wrapping_sub(glyph.offset_x) / 2;
		}
		self.width = max_width;
	}
}