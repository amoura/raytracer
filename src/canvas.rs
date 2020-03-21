use std::ops::{Add,Sub,Mul};
use crate::maths;

#[derive(Debug,Clone,Copy)]
pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Colour>,
}

impl PartialEq for Colour {
    fn eq(&self, other: &Self) -> bool {
        return maths::almost_same(self.red, other.red)
            && maths::almost_same(self.blue, other.blue) 
            && maths::almost_same(self.green, other.green) 
    }
}

impl Add for Colour {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Colour {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul for Colour {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
}

impl Colour {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {red, green, blue}
    }
}

const BLACK: Colour = Colour{red:0.0, green:0.0, blue:0.0};

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width, height,
            pixels: vec![Colour::new(0.0,0.0,0.0); width*height],
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Colour {
        self.pixels[y*self.width + x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Colour) {
        self.pixels[y*self.width + x] = c;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn colour() {
        let c1 = Colour::new(0.7, 1.1, 0.2);
        let c2 = Colour::new(0.1, 0.1, 0.7);

        assert_eq!(c1+c2, Colour::new(0.8, 1.2, 0.9));
        assert_eq!(c1-c2, Colour::new(0.6, 1.0, -0.5));
        assert_eq!(c1*0.5, Colour::new(0.35, 0.55, 0.1));
        assert_eq!(c1*c2, Colour::new(0.07, 0.11, 0.14));
    }

    #[test]
    fn canvas() {
        let mut c = Canvas::new(640, 480);
        c.write_pixel(55, 190, Colour::new(1.0, 0.5, 0.5));
        assert_eq!(c.pixel_at(55, 190), Colour::new(1.0, 0.5, 0.5));
        assert_eq!(c.pixel_at(54, 190), BLACK);
        assert_eq!(c.pixel_at(639, 479), BLACK);
    }
}