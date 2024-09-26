use std::{env, os::unix::thread, time::Duration};

mod draw;

fn main() {
    let args:Vec<String> = env::args().collect();
    let mut height:i32 = 20;
    let mut width:i32= 80;
    for arg in &args{
        if arg.contains("width"){
            width = arg.split("=").last().unwrap().parse::<i32>().unwrap();
        }
        if arg.contains("height"){
            height = arg.split("=").last().unwrap().parse::<i32>().unwrap();
        }
    }
    let mut surface : draw::Surface = draw::Surface::new(width as usize, (height*2) as usize);
    surface.clear();
    surface.draw_line(20, 20, 5, 0, draw::Color {r:0, g:231, b:95});
    surface.show();
}
