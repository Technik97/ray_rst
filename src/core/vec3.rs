
// Vec3 struct represents vector in 3D space
// We're using x, y and z coordinates for location.
#[derive(Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
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
        (u.x() * v.x()) + (u.y() * v.y()) + (u.z() + v.z())
    }

    pub fn cross_product(u: &Vec3, v: &Vec3) -> Self {
        // Cross product of 2 vectors.
        // About Cross product: https://www.mathsisfun.com/algebra/vectors-cross-product.html
        let x = u.y() * v.z() - u.z() * v.y();
        let y = u.z() * v.x() - u.x() * v.z();
        let z = u.x() * v.y() - u.y() * v.x();

        Self { x, y, z }
    }

    pub fn unit_vector(&self) -> Vec3 {
        // Returns unit vector of the given vector
        // About unit vector: https://www.mathsisfun.com/algebra/vector-unit.html
        let length = self.length();

        Self::new(self.x / length, self.y / length, self.z / length)
    }
}