

pub struct Surface{
    width : usize,
    height : usize,
    buffer : Vec<char>
}


impl Surface{
    fn new(width:usize, height:usize)->Surface{
        let buffer:Vec<char> = vec![' '; width*height];
        return Surface {width, height, buffer};
    }

    fn clear(&mut self){
        self.buffer = vec![' '; (self.width*self.height) as usize];
    }
}