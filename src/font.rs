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
	font:Font<'static>
}

impl GlyphMap
{
	pub fn load(size:usize,spacing:usize) -> GlyphMap
	{ 
		let font_data = include_bytes!("../ressources/Monaco.ttf");
		let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

		println!("metrics : {:?}",font.v_metrics( Scale::uniform(16.0)) );


		GlyphMap{
			size: size,
			spacing: spacing,
			font: font
		}
	}

	pub fn glyph_size(&self) -> (usize,usize)
	{
		(self.size,self.spacing)
	}

	pub fn glyph_from_char(&self,c:u8) -> Glyph
	{
		let mut text = String::new();
		text.push(c as char);

		let scale = Scale::uniform(self.size as f32);

		let v_metrics = self.font.v_metrics(scale);
		let glyphs: Vec<_> = self.font.layout(&text, scale, point(0.0, 0.0 + v_metrics.ascent)).collect();

		let min_x =  match glyphs.first(){
			Some(g) => {
				match g.pixel_bounding_box(){
					Some(bb) => bb.min.x,
					None => 0
				}
			},
			None => 0
		};	

		let max_x =  match glyphs.last(){
			Some(g) => {
				match g.pixel_bounding_box(){
					Some(bb) => bb.max.x,
					None => 0
				}
			}
			None => 0
		};	

		let glyphs_width =(max_x - min_x) as u32;

		let mut returned_glyph = Glyph{
			offest_x:(self.size-(glyphs_width as usize))/2,
			offest_y:0,
			data:vec![]
		};

		for glyph in glyphs 
		{
			if let Some(bounding_box) = glyph.pixel_bounding_box() 
			{
				returned_glyph.offest_y = if bounding_box.min.y < 0 { 0 } else { bounding_box.min.y as usize};
				glyph.draw(|x, y, v| { returned_glyph.data.push((x as usize,y as usize,v));});
			}
		}

		return returned_glyph;
	}
}