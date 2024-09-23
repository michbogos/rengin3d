use std::{env, os::unix::thread, time::Duration};

mod draw;

fn main() {
    let args:Vec<String> = env::args().collect();
    let mut height:usize = 20;
    let mut width:usize = 80;
    for arg in &args{
        if arg.contains("width"){
            width = arg.split("=").last().unwrap().parse::<usize>().unwrap();
        }
        if arg.contains("height"){
            height = arg.split("=").last().unwrap().parse::<usize>().unwrap();
        }
    }
    let mut surface : draw::Surface = draw::Surface::new(width, height);
    for x in 0..width{
        surface.clear();
        // draw::reset_cursor();
        surface.draw_line(0, 0, width-x-1, height-1);
        surface.show();
        std::thread::sleep(Duration::from_millis(33));
    }
}
