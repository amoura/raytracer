use std::ops::{Add,Sub,Mul};
use std::io;
use std::io::Write;
use std::fs::File;
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

    pub fn to_rgba(&self) -> u32 {
        (Self::to_255(self.red) << 8) |
            (Self::to_255(self.green) << 16) |
            (Self::to_255(self.blue) << 24) |
            255 // alpha = 1 for now
    }

    fn write(&self, file: &mut File) -> io::Result<()> {
        write_u8(file, Self::to_255_u8(self.blue))?;
        write_u8(file, Self::to_255_u8(self.green))?;
        write_u8(file, Self::to_255_u8(self.red))
    }

    fn to_255(x: f64) -> u32 {
        (x * 255.0) as u32
    }

    fn to_255_u8(x: f64) -> u8 {
        (x * 255.0) as u8
    }
}

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

    pub fn to_bmp(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;

        // BMP header
        write_u8(&mut file, 0x42)?;
        write_u8(&mut file, 0x4d)?;
        let bmp_header_size = 14;
        let dib_header_size = 40;
        let pixel_array_offset = bmp_header_size + dib_header_size;
        let bitmap_size = self.bmp_size();
        let bmp_size = pixel_array_offset + bitmap_size;

        write_u32(&mut file, bmp_size as u32)?;
        write_u32(&mut file, 0)?;
        write_u32(&mut file, pixel_array_offset as u32)?;

        // DIB header
        write_u32(&mut file, dib_header_size as u32)?;
        write_i32(&mut file, self.width as i32)?;
        write_i32(&mut file, self.height as i32)?;
        write_u16(&mut file, 1)?;
        write_u16(&mut file, 24)?; // bits per pixel
        write_u32(&mut file, 0)?;  // RGB format
        write_u32(&mut file, bitmap_size as u32)?;
        write_u32(&mut file, 2835)?; // resolution, 72 dpi
        write_u32(&mut file, 2835)?; // resolution, 72 dpi
        write_i32(&mut file, 0)?;  // spurious
        write_i32(&mut file, 0)?;  // spurious

        // Bitmap data
        for y in 0..self.height {
            for x in 0..self.width {
                self.pixel_at(x, y).write(&mut file)?;
            }
            for _ in 0..self.bmp_padding() {
                write_u8(&mut file, 0)?;
            }
        }

        Ok(())
    }

    fn bmp_padding(&self) -> usize {
        let row_size = self.width * 3;
        let rem = row_size % 4;
        if rem == 0 {
            0
        } else {
            4 - rem
        }
    }

    fn bmp_size(&self) -> usize {
        (self.width*3 + self.bmp_padding())*self.height
    }
}

fn write_u8(file: &mut File, val: u8) -> io::Result<()> {
    file.write_all(&u8::to_le_bytes(val))
}

fn write_u16(file: &mut File, val: u16) -> io::Result<()> {
    file.write_all(&u16::to_le_bytes(val))
}

fn write_u32(file: &mut File, val: u32) -> io::Result<()> {
    file.write_all(&u32::to_le_bytes(val))
}

fn write_i32(file: &mut File, val: i32) -> io::Result<()> {
    file.write_all(&i32::to_le_bytes(val))
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
        assert_eq!(c.pixel_at(54, 190), Colour::new(0.0, 0.0, 0.0));
        assert_eq!(c.pixel_at(639, 479), Colour::new(0.0, 0.0, 0.0));
    }
}