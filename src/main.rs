use std::{env, os::unix::thread, time::Duration};

mod draw;
mod linalg;

fn main() {
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
    surface.clear();
    surface.fill_circle(width/2, height/2, 30, draw::Color {r:128, g:65, b:234});
    surface.draw_triangle(0, 0, 10, 10, 10, 0, draw::Color {r:95, g:253, b:145});
    surface.show();
}
