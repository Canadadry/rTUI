pub type Color = u32;

pub fn mix(fg:Color,bg:Color,fg_opacity:f32) -> Color
{
	let fg = extract(fg);
	let bg = extract(bg);

	merge((
		((fg.0 as f32) * fg_opacity + (bg.0 as f32) * (1.0 - fg_opacity)) as u8,
		((fg.1 as f32) * fg_opacity + (bg.1 as f32) * (1.0 - fg_opacity)) as u8,
		((fg.2 as f32) * fg_opacity + (bg.2 as f32) * (1.0 - fg_opacity)) as u8
	))
}

fn extract(c:Color) -> (u8,u8,u8)
{
	let r = (c & 0xFF0000) >> 16;
	let g = (c & 0x00FF00) >>  8;
	let b = (c & 0x0000FF) >>  0;

	return (r as u8,g as u8,b as u8);
}

fn merge(c:(u8,u8,u8)) -> Color
{
	((c.0 as u32) << 16) | ((c.1 as u32) << 8) | (c.2 as u32)
}
