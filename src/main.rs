use std::env;

use camera::Camera;
use draw::reset_cursor;
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
    let mut cam:Camera = Camera::default();
    cam.near = 0.1;
    cam.far = 2.0;
    let proj = cam.projection_matrix();
    let verts = cube.map(|x|proj*x);
    reset_cursor();
    surface.clear();
    for i in 0..8{
        surface.draw_triangle((verts[i%8].x()*width as f32) as i32, (verts[i%8].y()*height as f32) as i32, (verts[(i+1)%8].x()*width as f32) as i32, (verts[(i+1)%8].y()*height as f32) as i32, (verts[(i+2)%8].x()*width as f32) as i32, (verts[(i+2)%8].y()*height as f32) as i32, draw::Color {r:95, g:253, b:145});
    }
    // surface.fill_circle(width/2, height/2, 30, draw::Color {r:128, g:65, b:234});
    // surface.fill_triangle(0, 0, 10, 10, 10, 0, draw::Color {r:95, g:253, b:145});
    surface.show();
}
