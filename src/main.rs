use std::env;

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
    surface.clear();
    for i in 0..165{
        for j in 0..10{
            if (i/4+j/4) % 2 == 0{
                surface.set(i, j);
            }
        }
    }
    surface.show();
}
