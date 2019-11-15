use color;
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
			bg:color::BLACK,
			fg:color::WHITE,
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
		for c in &self.commands
		{
			match c {
				Command::Write{x,y,content} => {
					let lines:Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
					let left              = x+ self.x + LEFT_BORDER_SIZE;
					let top               = y+ self.y + TOP_BORDER_SIZE;
					let available_width   = self.width  - x - LEFT_BORDER_SIZE - RIGHT_BORDER_SIZE;
					let available_height  = self.height - y - TOP_BORDER_SIZE  - BOTTOM_BORDER_SIZE;
					let lines_count       = std::cmp::min(lines.len(),available_height); 

					for i in 0..lines_count
					{
						let line           = &lines[i]; 
						let column_count   = std::cmp::min(line.len(),available_width); 
						let extract:String = line[..column_count].to_string();
	
						screen.draw_at(&extract,left,top+i,self.fg,self.bg);
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
		self.commands.clear();
		self.draw_border(screen);
	}

	fn draw_border(&mut self, screen:&mut screen::Screen)
	{
		for i in 0..self.width
		{
			screen.draw_at(&String::from("-"),self.x+i,self.y            ,self.fg,self.bg);
			screen.draw_at(&String::from("-"),self.x+i,self.y+self.height,self.fg,self.bg);
		}
		for j in 0..self.height
		{
			screen.draw_at(&String::from("|"),self.x           ,self.y+j,self.fg,self.bg);
			screen.draw_at(&String::from("|"),self.x+self.width,self.y+j,self.fg,self.bg);
		}
		screen.draw_at(&String::from("+"),self.x+self.width,self.y+self.height,self.fg,self.bg);
	}


}



