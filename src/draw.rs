use std::{io::Write, mem::swap};



pub struct Surface{
    width : usize,
    height : usize,
    buffer : Vec<char>,
    output: std::io::Stdout
}


impl Surface{
    pub fn new(width:usize, height:usize)->Surface{
        let buffer:Vec<char> = vec![' '; width*height];
        let output:std::io::Stdout = std::io::stdout();
        return Surface {width, height, buffer, output};
    }

    pub fn clear(&mut self){
        let size = self.buffer.len();
        self.buffer.clear();
        self.buffer.resize(size, ' ');
    }

    pub fn show(&mut self){
        let _ = self.output.write(self.buffer.iter().map(|c|c.to_string()).collect::<Vec<String>>().join("").as_bytes());
    }

    pub fn set(&mut self, x:i32, y:i32){
        if x as usize>=self.width{
            return;
        }
        if y as usize >=self.height{
            return;
        }
        self.buffer[y as usize *self.width+x as usize] = '▀';
    }

    pub fn draw_line(&mut self, mut x0:i32, mut y0:i32, mut x1:i32, mut y1:i32){
        let mut steep:bool = false;
        if x0.abs_diff(x1)<y0.abs_diff(y1){
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
            steep=true;
        }
        if x0 > x1{
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }
        let dx:i32 = x1 - x0;
        let dy:i32 = y1 - y0;
        let derror :i32 = (2*dy).abs();
        let mut error:i32 = 0;
        let mut y :i32 = y0;
        for x in x0..x1+1{
            if steep{
                self.set(y, x);
            }
            else{
                self.set(x, y);
            }
            error += derror;
            if error > dx{
                y += if(y1>y0){1} else{-1};
                error-=2*derror;
            }
        }
    }

    pub fn draw_circle(&mut self, x0:i32, y0:i32, r:i32){
        let mut d:i32 = 3-2*r as i32;
        let mut x:i32 = 0;
        let mut y:i32 = r;
        while y >= x{
            self.set(x+x0 , y+y0 );
            self.set(y+x0 , x+y0 );
            self.set(-y+x0, x+y0 );
            self.set(-x+x0, y+y0 );
            self.set(-x+x0, -y+y0);
            self.set(-y+x0, -x+y0);
            self.set(y+x0 , -x+y0);
            self.set(x+x0 , -y+y0);
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