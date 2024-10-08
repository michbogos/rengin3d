#![allow(dead_code)]

use std::io::Write;
use crate::linalg::Vecn;

#[derive(Copy, Clone)]
pub struct Color{
    pub r:u8,
    pub g:u8,
    pub b:u8,
}

pub struct Surface{
    width : usize,
    height : usize,
    buffer : Vec<Color>,
    output: std::io::Stdout
}


impl Surface{
    pub fn new(width:usize, height:usize)->Surface{
        let buffer:Vec<Color> = vec![Color {r:0, g:0, b:0}; width*height];
        let output:std::io::Stdout = std::io::stdout();
        return Surface {width, height, buffer, output};
    }

    pub fn clear(&mut self){
        let size = self.buffer.len();
        self.buffer.clear();
        self.buffer.resize(size, Color {r:0, g:0, b:0});
    }

    pub fn show(&mut self){
        let mut out_string: String = String::from("");
        for i in (0..self.height).step_by(2){
            for j in 0..self.width{
                let col1:Color = self.buffer[i*self.width+j];
                let col2:Color = self.buffer[(i+1)*self.width+j];
                out_string.push_str(ansi_truecolor(col1, col2, '▀').as_str());
            }
        }
        let _ = self.output.write(out_string.as_bytes());
        let _ = self.output.flush();
    }

    pub fn set(&mut self, x:i32, y:i32, color:Color){
        if x as usize>=self.width{
            return;
        }
        if y as usize >=self.height{
            return;
        }
        self.buffer[y as usize *self.width+x as usize] = color;
    }

    pub fn draw_line(&mut self, mut x0:i32, mut y0:i32, x1:i32, y1:i32, col:Color){
        let dx:i32 = x0.abs_diff(x1) as i32;
        let dy:i32 = -(y0.abs_diff(y1) as i32);
        let sx:i32 = if x0<x1 {1} else {-1};
        let sy:i32 = if y0<y1 {1} else {-1};
        let mut err:i32 = dx+dy;
        let mut e2:i32;
        while !(x0==x1 && y0==y1){
            self.set(x0, y0, col);
            e2 = 2*err;
            if e2 >= dy { err += dy; x0 += sx; } /* e_xy+e_x > 0 */
            if e2 <= dx { err += dx; y0 += sy; }
        }
    }

    pub fn draw_circle(&mut self, x0:i32, y0:i32, r:i32, col:Color){
        let mut d:i32 = 3-2*r as i32;
        let mut x:i32 = 0;
        let mut y:i32 = r;
        while y >= x{
            self.set(x+x0 , y+y0 , col);
            self.set(y+x0 , x+y0 , col);
            self.set(-y+x0, x+y0 , col);
            self.set(-x+x0, y+y0 , col);
            self.set(-x+x0, -y+y0, col);
            self.set(-y+x0, -x+y0, col);
            self.set(y+x0 , -x+y0, col);
            self.set(x+x0 , -y+y0, col);
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

    pub fn fill_circle(&mut self, x0:i32, y0:i32, r:i32, col:Color){
        for i in y0-r..y0+r{
            for j in x0-r..x0+r{
                if (j-x0)*(j-x0) + (i-y0)*(i-y0) < r*r{
                    self.set(j, i, col);
                }
            }
        }
    }

    pub fn draw_triangle(&mut self, ax:i32, ay:i32, bx:i32, by:i32, cx:i32, cy:i32, col:Color){
        self.draw_line(ax, ay, bx, by, col);
        self.draw_line(bx, by, cx, cy, col);
        self.draw_line(cx, cy, ax, ay, col);
    }

    pub fn fill_triangle(&mut self, ax:i32, ay:i32, bx:i32, by:i32, cx:i32, cy:i32, col:Color){
        let a : Vecn<2, f32> = Vecn::<2, f32> {data:[(ax as f32), ay as f32]};
        let b : Vecn<2, f32> = Vecn::<2, f32> {data:[(bx as f32), by as f32]};
        let c : Vecn<2, f32> = Vecn::<2, f32> {data:[(cx as f32), cy as f32]};

        let minx:i32 = ax.min(bx).min(cx);
        let maxx:i32 = ax.max(bx).max(cx);

        let miny:i32 = ay.min(by).min(cy);
        let maxy:i32 = ay.max(by).max(cy);

        let v0 = b-a;
        let v1 = c-a;

        let d00 = v0*v0;
        let d01 = v0*v1;
        let d11 = v1*v1;

        for i in minx .. maxx{
            for j in miny .. maxy{
                let p: Vecn<2, f32> = Vecn::<2, f32> {data:[i as f32, j as f32]};
                let v2 = p-a;
                let d20 = v2*v0;
                let d21 = v2*v1;
                let denom = d00 * d11 - d01 * d01;
                let v = (d11 * d20 - d01 * d21) / denom;
                let w = (d00 * d21 - d01 * d20) / denom;
                let u = 1.0 - v - w;
                if v > 0.0 && w > 0.0 && u > 0.0{
                    self.set(i, j, col);
                }
            }
        }

    }

}

pub fn reset_cursor(){
    print!("\x1b\x63");
}

fn ansi_truecolor(fg:Color, bg:Color, c:char)->String{
    return format!("\x1b[38;2;{};{};{};48;2;{};{};{}m{}", fg.r, fg.g, fg.b, bg.r, bg.g, bg.b, c)
}