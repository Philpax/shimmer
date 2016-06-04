use glium::texture::PixelValue;
use glium::texture::ClientFormat;

use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Colour {
        Colour {
            r: r,
            g: g,
            b: b,
            a: a,
        }        
    }

    pub fn zero() -> Colour {
        Colour {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    pub fn white() -> Colour {
        Colour {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

impl Mul<f32> for Colour {
    type Output = Colour;

    fn mul(self, other: f32) -> Colour {
        Colour {
            r: (self.r as f32 * other) as u8,
            g: (self.g as f32 * other) as u8,
            b: (self.b as f32 * other) as u8,
            a: self.a,
        }
    }
}

unsafe impl PixelValue for Colour {
    fn get_format() -> ClientFormat {
        return ClientFormat::U8U8U8U8;
    }
}

pub struct Point {
    pub value: f32,
    pub colour: Colour,
}
