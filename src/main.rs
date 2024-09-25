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
    let mut surface : draw::Surface = draw::Surface::new(width as usize, height as usize);
    surface.draw_line(0, 0, width, height);
    surface.show();
}
