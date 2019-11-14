use lodepng;

pub struct Glyph
{
	glyph_x:usize,
	glyph_y:usize,
	iter_x:usize,
	iter_y:usize,
	size:usize
}

impl Iterator for Glyph {
    type Item = (usize,usize,usize);
    
    fn next(&mut self) -> Option<(usize,usize,usize)> {
    	if self.iter_x >= self.size {
    		self.iter_x = 0;
    		self.iter_y += 1;
    	}
    	if self.iter_y >= self.size {
    		return None;
    	}
    	let pos_y    = (self.glyph_y*self.size)+self.iter_y;
    	let pos_x    = (self.glyph_x*self.size)+self.iter_x; 
    	let pos      = pos_y*16*self.size+pos_x;
    	let out      = (self.iter_x,self.iter_y,pos);
    	self.iter_x += 1;

        return Some(out);
    }
}

pub struct GlyphMap
{
	size:usize,
	map:Vec<bool>
}

impl GlyphMap
{
	pub fn load_png(filename:&str) -> GlyphMap
	{ 
		let img = lodepng::decode32_file(filename).unwrap();

		if img.width != img.height  { panic!("Font image must be a square"); }
		if img.width % 16 != 0 { panic!("Font image size must be a multiple of 16"); }
		let size = (img.width/16) as usize;

		let mut glyph_map = GlyphMap{
			size: size,
			map: vec![false;256*size*size]
		};

		for i in 0..img.buffer.len()
		{
			if i%128 == 0 { print!("\n");}

			let p = img.buffer[i].rgb();
			let out:bool =  (0.299*(p.r as f32)/255.0+ 0.587*(p.g as f32)/255.0 +0.114*(p.b as f32)/255.0 ) > 0.5;
			glyph_map.map[i]=out;
		}

		return glyph_map;
	}

	pub fn glyph_size(&self) -> usize
	{
		return self.size;
	}

	pub fn glyph_from_char(&self,char:u8) -> Glyph
	{
		Glyph
		{
			glyph_x:(char as usize)%16,
			glyph_y:(char as usize)/16,
			iter_x:0,
			iter_y:0,
			size:self.size
		}
	}

	pub fn pixel_from_id(&self,id:usize) -> Option<bool>
	{
		if id >= self.map.len() {
			None 
		} else {
			Some(self.map[id])
		}
	}
}