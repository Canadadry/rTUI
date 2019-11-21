use screen::Screen;

pub struct SubScreen {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
    width:usize,
    pos_id:Option<usize>,
    pos_x:usize,
    pos_y:usize
}

impl SubScreen {
    pub fn from(left:usize,width:usize,top:usize,height:usize,screen:&Screen,) -> Option<SubScreen>
    {
        let right = left + width;
        let bottom = top + height;
        if right  >= screen.real_width() { return None; }
        if bottom >= screen.real_height() { return None; }

        return Some(SubScreen{
            left:left,
            right:right,
            top:top,
            bottom:bottom,
            width:screen.real_width(),
            pos_id:Some(left+top*screen.real_width()),
            pos_x:0,
            pos_y:0
        });
    }

    fn pos(&self) -> Option<usize>
    {
        if (self.top  + self.pos_y) >= self.bottom
        {
            return None;
        } 

        let x = self.left + self.pos_x;
        let y = self.top  + self.pos_y;
        let w = self.width;
        return Some(x+y*w);
    }

    fn increment_position(&mut self)
    {
        self.pos_x += 1;
        if (self.left + self.pos_x) >= self.right
        {
            self.pos_x = 0;
            self.pos_y += 1; 
        }
        if (self.top  + self.pos_y) >= self.bottom
        {
            self.pos_id = None;
        } 
    }
}

impl Iterator for SubScreen {
    type Item = (usize,usize,usize);
    
    fn next(&mut self) -> Option<Self::Item> 
    {
        let mut next:Option<Self::Item> = None;
        let pos = self.pos();

        if let Some(pos) = pos
        {
            next = Some((self.pos_x,self.pos_y,pos));
        }
        self.increment_position();
        return next;
    }
}