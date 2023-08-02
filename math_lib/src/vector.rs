#![allow(dead_code)]

use std::ops;

#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn identity() {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
    }

    pub fn eq(&self, other: Vec3) -> bool {
        (self.x == other.x) & (self.y == other.y) & (self.z == other.z)
    }

    pub fn not_eq(&self, other: Vec3) -> bool {
        (self.x != other.x) | (self.y != other.y) | (self.z != other.z)
    }

    pub fn neg(&self) {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }

    pub fn add(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn normalize(&mut self) {
        let mag_sq = self.x * self.x + self.y * self.y + self.z * self.z;
        if mag_sq > 0.0 {
            let one_over_mag = 1.0 / mag_sq.sqrt();
            self.x *= one_over_mag;
            self.y *= one_over_mag;
            self.z *= one_over_mag;
        }
    }

    // dot product
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

pub fn cross_product(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

pub fn distance(a: &Vec3, b: &Vec3) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// Scalar multiple
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, a: f32) -> Self::Output {
        Vec3 {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

// Scalar divide
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, a: f32) -> Self::Output {
        Vec3 {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// Scalar *=
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, a: f32) {
        self.x *= a;
        self.y *= a;
        self.z *= a;
    }
}

// Scalar /=
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, a: f32) {
        self.x *= a;
        self.y *= a;
        self.z *= a;
    }
}
