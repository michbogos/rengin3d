

pub struct Surface{
    width : usize,
    height : usize,
    buffer : Vec<char>
}


impl Surface{
    pub fn new(width:usize, height:usize)->Surface{
        let buffer:Vec<char> = vec![' '; width*height];
        return Surface {width, height, buffer};
    }

    pub fn clear(&mut self){
        self.buffer = vec![' '; (self.width*self.height) as usize];
    }

    pub fn show(self){
        for c in self.buffer {
            print!("{}", c);
        }
    }

    pub fn set(&mut self, x:usize, y:usize){
        if x>=self.width{
            panic!("Attempted set out of x range\n");
        }
        if y>=self.height{
            panic!("Attempted set out of y range\n");
        }
        self.buffer[y*self.width+x] = '#';
    }
}