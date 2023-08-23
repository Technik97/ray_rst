use std::{ops::{Mul, Add, Sub, Div, Neg}};

// Vec3 struct represents vector in 3D space
// We're using x, y and z coordinates for location.
#[derive(Debug, Default, Copy, Clone, serde::Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        // Create a new Vector
        Self { x, y, z}
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn length_squared(&self) -> f32 {
        (self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z()) 
    }

    pub fn length(&self) -> f32 {
        // returns the length of the vector
        self.length_squared().sqrt()
    }

    pub fn dot_product(u: &Vec3, v: &Vec3) -> f32 {
        // Dot product of 2 vectors.
        // About Dot product: https://www.mathsisfun.com/algebra/vectors-dot-product.html
        (u.x() * v.x()) + (u.y() * v.y()) + (u.z() * v.z())
    }

    pub fn cross_product(u: &Vec3, v: &Vec3) -> Self {
        // Cross product of 2 vectors.
        // About Cross product: https://www.mathsisfun.com/algebra/vectors-cross-product.html
        let x: f32 = u.y() * v.z() - u.z() * v.y();
        let y: f32 = u.z() * v.x() - u.x() * v.z();
        let z: f32 = u.x() * v.y() - u.y() * v.x();

        Self { x, y, z }
    }

    pub fn unit_vector(&self) -> Vec3 {
        // Returns unit vector of the given vector
        // About unit vector: https://www.mathsisfun.com/algebra/vector-unit.html
        let length: f32 = self.length();

        Self::new(self.x / length, self.y / length, self.z / length)
    }
}


impl Mul<f32> for Vec3 {
    // Implementing Mul trait for Vec3
    // Because we'll be going to multiply a vector in at() fn 
    // which returns the location of a vector.
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x() * rhs,
            y: self.y() * rhs,
            z: self.z() * rhs
        }
    }
}

impl Add for Vec3 {
    // Implementing Add trait for Vec3
    // Because we'll be going to add 2 vectors  
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
            z: self.z() + rhs.z(),
        }    
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
            z: self.z() - rhs.z()
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x() / rhs,
            y: self.y() / rhs,
            z: self.z() / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
