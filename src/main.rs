use raytracer::canvas::Canvas;
use raytracer::canvas::Colour;

fn paint_square(c: &mut Canvas, col: Colour, x0: usize, y0: usize, x1: usize, y1: usize) {
    for y in y0..y1 {
        for x in x0..x1 {
            c.write_pixel(x, y, col);
        }
    }
}

fn main() {
    let mut c = Canvas::new(512, 512);
    let blue = Colour::new(0.0, 0.0, 1.0);
    let red = Colour::new(1.0, 0.0, 0.0);
    paint_square(&mut c, blue, 0, 0, 128, 128);
    paint_square(&mut c, red, 384, 384, 512, 512);
    paint_square(&mut c, blue, 450, 0, 512, 62);
//    c.to_bmp("tst.bmp").expect("Failed to create .bmp file");
}
