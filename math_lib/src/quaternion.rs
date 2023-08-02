#![allow(dead_code)]

use crate::euler_angles::EulerAngles;
use crate::vector::Vec3;
use std::ops;

#[derive(Debug)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn identity() -> Quaternion {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn set_to_rotate_about_x(theta: f32) -> f32 {
        todo!()
    }

    pub fn set_to_rotate_about_y(theta: f32) {
        todo!()
    }

    pub fn set_to_rotate_about_z(theta: f32) {
        todo!()
    }

    pub fn set_to_rotate_about_axis(axis: Vec3, theta: f32) {
        todo!()
    }

    // Setup to perform object<->inertial rotations,
    // given orientation in Euler angle format

    pub fn set_to_rotate_object_to_inertial(orientation: EulerAngles) {
        todo!()
    }

    pub fn set_to_rotate_inertial_to_object(orientation: EulerAngles) {
        todo!()
    }

    // Normalize the quaternion.
    pub fn normalize(&self) {
        todo!()
    }

    // Extract and return the rotation angle and axis.
    pub fn get_rotation_angle() -> f32 {
        todo!()
    }
    pub fn get_rotation_axis() -> Vec3 {
        todo!()
    }
}

// Cross product
impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, a: Quaternion) -> Self::Output {
        todo!();
        Quaternion {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 0.0,
        }
    }
}

// Multiplication with assignment, as per C++ convention
impl ops::MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, rhs: Quaternion) {
        todo!()
    }
}
