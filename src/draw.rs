use std::io::Write;



pub struct Surface{
    width : usize,
    height : usize,
    buffer : Vec<u8>,
    output: std::io::Stdout
}


impl Surface{
    pub fn new(width:usize, height:usize)->Surface{
        let buffer:Vec<u8> = vec![' ' as u8; width*height];
        let output:std::io::Stdout = std::io::stdout();
        return Surface {width, height, buffer, output};
    }

    pub fn clear(&mut self){
        let size = self.buffer.len();
        self.buffer.clear();
        self.buffer.resize(size, ' ' as u8);
    }

    pub fn show(&mut self){
        let _ = self.output.write(self.buffer.as_slice());
    }

    pub fn set(&mut self, x:usize, y:usize){
        if x>=self.width{
            panic!("Attempted set out of x range\n");
        }
        if y>=self.height{
            panic!("Attempted set out of y range\n");
        }
        self.buffer[y*self.width+x] = '#' as u8;
    }

    pub fn draw_line(&mut self, x0:usize, y0:usize, x1:usize, y1:usize){
        let dx:i32 = x1 as i32 - x0 as i32;
        let dy:i32 = y1 as i32 - y0 as i32;
        let mut d :i32 = 2*dy - dx;
        let mut y = y0;
        for x in x0..x1{
            self.set(x, y);
            if d > 0{
                y = y+1;
                d=d-2*dx;
            }
            d+=2*dy;
        }
    }

    pub fn draw_circle(&mut self, x0:i32, y0:i32, r:i32){
        let mut d:i32 = 3-2*r as i32;
        let mut x:i32 = 0;
        let mut y:i32 = r;
        while y >= x{
            self.set((x+x0 ) as usize, (y+y0)  as usize);
            self.set((y+x0 ) as usize, (x+y0)  as usize);
            self.set((-y+x0) as usize, (x+y0)  as usize);
            self.set((-x+x0) as usize, (y+y0)  as usize);
            self.set((-x+x0) as usize, (-y+y0) as usize);
            self.set((-y+x0) as usize, (-x+y0) as usize);
            self.set((y+x0 ) as usize, (-x+y0) as usize);
            self.set((x+x0 ) as usize, (-y+y0) as usize);
            if d > 0{
                d += 4*(x-y)+10;
                y -=1;
            }
            else {
                d += 4*x+6;
            }
            x += 1;
        }
    }
}

pub fn reset_cursor(){
    print!("\0033\0143");
}