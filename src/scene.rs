use glium::texture::PixelValue;
use glium::texture::ClientFormat;

use std::ops::Mul;
use cgmath::*;

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

pub struct SignedDistance {
    pub value: f32,
    pub colour: Colour,
}

impl SignedDistance {
    pub fn union(self, other: SignedDistance) -> SignedDistance {
        if other.value < self.value {
            other
        } else {
            self
        }
    }
}

// Objects
// Object
pub trait Object {
    fn evaluate(&self, point: Point3<f32>) -> SignedDistance;
}

// Sphere
pub struct Sphere {
    centre: Point3<f32>,
    radius: f32,
    colour: Colour,
}

impl Sphere {
    pub fn new(centre: Point3<f32>, radius: f32, colour: Colour) -> Sphere {
        Sphere {
            centre: centre,
            radius: radius,
            colour: colour
        }
    }
}

impl Object for Sphere {
    fn evaluate(&self, point: Point3<f32>) -> SignedDistance {
        SignedDistance {
            value: (point - self.centre).magnitude() - self.radius,
            colour: self.colour,
        }
    }
}

// Plane
pub struct Plane {
    normal: Vector3<f32>,
    determinant: f32,
    colour: Colour
}

impl Plane {
    pub fn new(normal: Vector3<f32>, point: Point3<f32>, colour: Colour) -> Plane {
        Plane {
            normal: normal,
            determinant: -(normal.x * point.x + normal.y * point.y + normal.z * point.z),
            colour: colour,
        }
    }
}

impl Object for Plane {
    fn evaluate(&self, point: Point3<f32>) -> SignedDistance {
        SignedDistance {
            value: self.normal.extend(self.determinant).dot(point.to_homogeneous()),
            colour: self.colour,
        }
    }
}