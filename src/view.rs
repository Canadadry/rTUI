use color;
use color_table;
use screen;

const LEFT_BORDER_SIZE:usize   = 1;
const RIGHT_BORDER_SIZE:usize  = 1;
const TOP_BORDER_SIZE:usize    = 1;
const BOTTOM_BORDER_SIZE:usize = 1;

enum Command
{
	Write{
		x:usize,
		y:usize,
		content:String
	},
	Stream{
		content:String
	},
	Color
	{
		bg:color::Color,
		fg:color::Color
	},
	Clear
	{
		x:usize,
		y:usize,
		width:usize,
		height:usize
	}
}

pub struct View
{
	x:usize,
	y:usize,
	width:usize,
	height:usize,
	bg:color::Color,
	fg:color::Color,
	commands:Vec<Command>,
}

impl View 
{
	pub fn new(x:usize,y:usize,width:usize,height:usize) -> View 
	{
		View{
			x:x,
			y:y,
			width:width,
			height:height,
			bg:color_table::BLACK,
			fg:color_table::WHITE,
			commands:vec![Command::Clear{x:0,y:0,width:width,height:height}]
		}
	}

	pub fn draw_at(&mut self, string:String,x:usize,y:usize)
	{
		self.commands.push(Command::Write{
			x:x,
			y:y,
			content:string
		});
	}

	pub fn stream(&mut self, string:String)
	{
		self.commands.push(Command::Stream{
			content:string
		});
	}

	pub fn color(&mut self, bg:color::Color, fg:color::Color)
	{
		self.commands.push(Command::Color{
			bg:bg,
			fg:fg
		});
	}

	pub fn clear(&mut self)
	{
		self.commands.push(Command::Clear{
			x:0,
			y:0,
			width:self.width,
			height:self.height
		});
	}

	pub fn apply(&mut self, screen:&mut screen::Screen) 
	{
		let mut stream = (0usize,0usize);

		for c in &self.commands
		{
			match c {
				Command::Write{x,y,content} => {
					let lines:Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
					let left              = x+ self.x + LEFT_BORDER_SIZE;
					let top               = y+ self.y + TOP_BORDER_SIZE;
					let available_width   = self.width  - x - LEFT_BORDER_SIZE - RIGHT_BORDER_SIZE ;
					let available_height  = self.height - y - TOP_BORDER_SIZE  - BOTTOM_BORDER_SIZE ;
					let lines_count       = std::cmp::min(lines.len(),available_height); 

					for i in 0..lines_count
					{
						let line           = &lines[i]; 
						let column_count   = std::cmp::min(line.len(),available_width); 
						let extract:String = line[..column_count].to_string();
	
						screen.draw_at(&extract,left,top+i,self.fg,self.bg);
					}
				},
				Command::Stream{content} => {
					for c in content.chars()
					{
						if stream.1 >= (self.height - TOP_BORDER_SIZE - BOTTOM_BORDER_SIZE){
							break;
						}
 						if c == b'\n' as char {
 							stream.0 = 0;
 							stream.1 += 1; 
 							continue;
 						}
						let left  = stream.0 + self.x + LEFT_BORDER_SIZE;
						let top   = stream.1+ self.y + TOP_BORDER_SIZE;	
						screen.draw_at(&c.to_string(),left,top,self.fg,self.bg);
 
						stream.0 += 1;
						if stream.0 >= (self.width - LEFT_BORDER_SIZE - RIGHT_BORDER_SIZE)
						{
 							stream.0 = 0;
 							stream.1 += 1; 
 							continue;
						}
					}
				},
				Command::Clear{x,y,width,height} => {
					for i in 0..*width
					{
						for j in 0..*height
						{
							screen.draw_at(&String::from(" "),self.x+x+i,self.y+y+j,self.fg,self.bg);
						}		
					}
				},
				Command::Color{fg,bg} => {
					self.fg = *fg;
					self.bg = *bg;
				}
			}
		}
		if !self.commands.is_empty()
		{
			self.draw_border(screen);
		}
		self.commands.clear();
	}

	fn draw_border(&mut self, screen:&mut screen::Screen)
	{
		for i in 0..self.width
		{
			screen.draw_at(&String::from("#"),self.x+i,self.y            ,self.fg,self.bg);
			screen.draw_at(&String::from("#"),self.x+i,self.y+self.height-1,self.fg,self.bg);
		}
		for j in 0..self.height
		{
			screen.draw_at(&String::from("#"),self.x           ,self.y+j,self.fg,self.bg);
			screen.draw_at(&String::from("#"),self.x+self.width-1,self.y+j,self.fg,self.bg);
		}
		screen.draw_at(&String::from("#"),self.x+self.width-1,self.y+self.height-1,self.fg,self.bg);
	}


}



