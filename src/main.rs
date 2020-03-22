use raytracer::canvas::Canvas;
use raytracer::canvas::Colour;
use raytracer::maths::{Matrix4,Matrix3,almost_same};

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

    let m3 = Matrix3::new(
        1.0, 2.0, 6.0,
        -5.0, 8.0, -4.0,
        2.0, 6.0, 4.0);
    assert!(almost_same(m3.cofactor(0,0), 56.0));        
    assert!(almost_same(m3.cofactor(0,1), 12.0));        
    let cof = m3.cofactor(0,2);
    assert!(almost_same(cof, -46.0));        
    assert!(almost_same(m3.det(), -196.0));  

    let m4 = Matrix4::new(
        -2.0 , -8.0 ,  3.0 ,  5.0 ,
        -3.0 ,  1.0 ,  7.0 ,  3.0 ,
        1.0 ,  2.0 , -9.0 ,  6.0 ,
        -6.0 ,  7.0 ,  7.0 , -9.0);
    assert!(almost_same(m4.cofactor(0,0), 690.0));        
    assert!(almost_same(m4.cofactor(0,1), 447.0));        
    assert!(almost_same(m4.cofactor(0,2), 210.0));        
    assert!(almost_same(m4.cofactor(0,3), 51.0));        
    assert!(almost_same(m4.det(), -4071.0));  

}
