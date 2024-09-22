mod draw;

fn main() {
    let mut surface : draw::Surface = draw::Surface::new(165, 10);
    surface.clear();
    for i in 0..165{
        for j in 0..10{
            if (i+j) % 2 == 0{
                surface.set(i, j);
            }
        }
    }
    surface.show();
}
