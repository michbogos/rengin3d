use std::env;
use std::thread;

use camera::Camera;
use draw::reset_cursor;
use linalg::Matrix;
use crate::linalg::Vecn;

mod draw;
mod linalg;
mod camera;

fn main() {
    let cube:[Vecn<4, f32>;8] = [ Vecn {data:[0.5, 0.5, 0.5, 0.0]},
                                    Vecn {data:[1., 0.5, 0.5, 0.0]},
                                    Vecn {data:[1., 1., 1., 0.0]},
                                    Vecn {data:[0.5, 1., 1., 0.0]},
                                    Vecn {data:[0.5, 0.5, 1., 0.0]},
                                    Vecn {data:[0.5, 1., 0.5, 0.0]},
                                    Vecn {data:[1., 0.5, 1., 0.0]},
                                    Vecn {data:[1., 1., 0.5, 0.0]},];
    let args:Vec<String> = env::args().collect();
    let mut height:i32 = 20;
    let mut width:i32= 80;
    for arg in &args{
        if arg.contains("width"){
            width = arg.split("=").last().unwrap().parse::<i32>().unwrap();
        }
        if arg.contains("height"){
            height = arg.split("=").last().unwrap().parse::<i32>().unwrap()*2;
        }
    }
    let mut surface : draw::Surface = draw::Surface::new(width as usize, height as usize);
    let mut verts:[Vecn<2, f32>;3] = [Vecn{data:[0., 0.]},Vecn{data:[10., 10.]}, Vecn{data:[10., 0.]}];
    for i in 0..360{
        let mut rotation = Matrix::rotation(3.1415926/360.0 as f32);
        surface.clear();
        reset_cursor();
        verts[0] = rotation*verts[0];
        verts[1] = rotation*verts[1];
        verts[2] = rotation*verts[2];
    // surface.fill_circle(width/2, height/2, 30, draw::Color {r:128, g:65, b:234});
        surface.fill_triangle(verts[0].x() as i32 + 10, verts[0].y() as i32 + 10, verts[1].x() as i32 + 10, verts[1].y() as i32 + 10, verts[2].x() as i32 + 10, verts[2].y() as i32 +10, draw::Color {r:95, g:253, b:145});
        surface.draw_triangle(verts[0].x() as i32 + 10, verts[0].y() as i32 + 10, verts[1].x() as i32 + 10, verts[1].y() as i32 + 10, verts[2].x() as i32 + 10, verts[2].y() as i32 +10, draw::Color {r:95, g:253, b:145});
        surface.show();
        thread::sleep(std::time::Duration::from_millis(80));
    }
}
