#![allow(dead_code)]

use crate::euler_angles::EulerAngles;
use crate::utils::{atan2, safe_acos};
use crate::vector3::Vector3;
use std::ops;

#[derive(Clone, Debug)]
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

    pub fn set_to_rotate_about_x(&mut self, theta: f32) {
        // Compute the half angle
        let theta_over_2 = theta * 0.5;

        self.w = theta_over_2.cos();
        self.x = theta_over_2.sin();
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn set_to_rotate_about_y(&mut self, theta: f32) {
        // Compute the half angle

        let theta_over_2 = theta * 0.5;

        // Set the values

        self.w = theta_over_2.cos();
        self.x = 0.0;
        self.y = theta_over_2.sin();
        self.z = 0.0;
    }

    pub fn set_to_rotate_about_z(&mut self, theta: f32) {
        // Compute the half angle

        let theta_over_2 = theta * 0.5;

        // Set the values

        self.w = theta_over_2.cos();
        self.x = 0.0;
        self.y = 0.0;
        self.z = theta_over_2.sin();
    }

    pub fn set_to_rotate_about_axis(&mut self, axis: Vector3, theta: f32) {
        // The axis of rotation must be normalized

        assert!((axis.magnitude() - 1.0).abs() < 0.01);

        // Compute the half angle and its sin

        let theta_over_2 = theta * 0.5;
        let sin_theta_over_2 = theta_over_2.sin();

        // Set the values

        self.w = theta_over_2.cos();
        self.x = axis.x * sin_theta_over_2;
        self.y = axis.y * sin_theta_over_2;
        self.z = axis.z * sin_theta_over_2;
    }

    // Setup the quaternion to perform an object->inertial rotation, given the
    // orientation in Euler angle format
    pub fn set_to_rotate_object_to_inertial(&mut self, orientation: EulerAngles) {
        let (sin_pitch, cos_pitch) = (orientation.pitch * 0.5).sin_cos();
        let (sin_bank, cos_bank) = (orientation.bank * 0.5).sin_cos();
        let (sin_heading, cos_heading) = (orientation.heading * 0.5).sin_cos();

        self.w = cos_heading * cos_pitch * cos_bank + sin_heading * sin_pitch * sin_bank;
        self.x = cos_heading * sin_pitch * cos_bank + sin_heading * cos_pitch * sin_bank;
        self.y = -cos_heading * sin_pitch * sin_bank + sin_heading * cos_pitch * cos_bank;
        self.z = -sin_heading * sin_pitch * cos_bank + cos_heading * cos_pitch * sin_bank;
    }

    // Setup the quaternion to perform an object->inertial rotation, given the
    // orientation in Euler angle format
    pub fn set_to_rotate_inertial_to_object(&mut self, orientation: EulerAngles) {
        let (sin_pitch, cos_pitch) = (orientation.pitch * 0.5).sin_cos();
        let (sin_bank, cos_bank) = (orientation.bank * 0.5).sin_cos();
        let (sin_heading, cos_heading) = (orientation.heading * 0.5).sin_cos();

        self.w = cos_heading * cos_pitch * cos_bank + sin_heading * sin_pitch * sin_bank;
        self.x = -cos_heading * sin_pitch * cos_bank - sin_heading * cos_pitch * sin_bank;
        self.y = cos_heading * sin_pitch * sin_bank - sin_heading * cos_bank * cos_pitch;
        self.z = sin_heading * sin_pitch * cos_bank - cos_heading * cos_pitch * sin_bank;
    }

    // Quaternion::normalize
    //
    // "Normalize" a quaternion.  Note that normally, quaternions
    // are always normalized (within limits of numerical precision).
    // See section 10.4.6 for more information.
    //
    // This function is provided primarily to combat floating point "error
    // creep," which can occur when many successive quaternion operations
    // are applied.
    pub fn normalize(&mut self) {
        // Compute magnitude of the quaternion

        let mag = (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

        // Check for bogus length, to protect against divide by zero
        if mag > 0.0 {
            // Normalize it
            let one_over_mag = 1.0 / mag;
            self.w *= one_over_mag;
            self.x *= one_over_mag;
            self.y *= one_over_mag;
            self.z *= one_over_mag;
        } else {
            // Houston, we have a problem
            assert!(false);

            // In a release build, just slam it to something
            // Quaternion::identity()
            self.x = 0.0;
            self.y = 0.0;
            self.z = 0.0;
            self.w = 1.0;
        }
    }

    // Quaternion::getRotationAngle
    // Return the rotation angle theta
    pub fn get_rotation_angle(&self) -> f32 {
        // Compute the half angle.  Remember that w = cos(theta / 2)
        let theta_over2 = safe_acos(self.w);

        // Return the rotation angle
        return theta_over2 * 2.0;
    }

    // Quaternion::getRotationAxis
    // Return the rotation axis
    pub fn get_rotation_axis(&self) -> Vector3 {
        // Compute sin^2(theta/2).  Remember that w = cos(theta/2),
        // and sin^2(x) + cos^2(x) = 1

        let sin_theta_over_2sq = 1.0 - self.w * self.w;

        // Protect against numerical imprecision

        if sin_theta_over_2sq <= 0.0 {
            // Identity quaternion, or numerical imprecision.  Just
            // return any valid vector, since it doesn't matter

            return Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
        }

        // Compute 1 / sin(theta/2)

        let one_over_sin_theta_over2 = 1.0 / sin_theta_over_2sq.sqrt();

        // Return axis of rotation

        return Vector3 {
            x: self.x * one_over_sin_theta_over2,
            y: self.y * one_over_sin_theta_over2,
            z: self.z * one_over_sin_theta_over2,
        };
    }
}

// Quaternion::operator *
//
// Quaternion cross product, which concatenates multiple angular
// displacements.  The order of multiplication, from left to right,
// corresponds to the order that the angular displacements are
// applied.  This is backwards from the *standard* definition of
// quaternion multiplication.  See section 10.4.8 for the rationale
// behind this deviation from the standard.
impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, a: Quaternion) -> Self::Output {
        Quaternion {
            w: self.w * a.w - self.x * a.x - self.y * a.y - self.z * a.z,
            x: self.w * a.x + self.x * a.w + self.z * a.y - self.y * a.z,
            y: self.w * a.y + self.y * a.w + self.x * a.z - self.z * a.x,
            z: self.w * a.z + self.z * a.w + self.y * a.x - self.x * a.y,
        }
    }
}

// Quaternion::operator *=
// Combined cross product and assignment, as per C++ convention
impl ops::MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, a: Quaternion) {
        self.w = self.w * a.w - self.x * a.x - self.y * a.y - self.z * a.z;
        self.x = self.w * a.x + self.x * a.w + self.z * a.y - self.y * a.z;
        self.y = self.w * a.y + self.y * a.w + self.x * a.z - self.z * a.x;
        self.w = self.w * a.z + self.z * a.w + self.y * a.x - self.x * a.y;
    }
}

//---------------------------------------------------------------------------
// dotProduct
//
// Quaternion dot product.  We use a nonmember function so we can
// pass quaternion expressions as operands without having "funky syntax"
//
// See 10.4.10

pub fn dot_product(a: &Quaternion, b: &Quaternion) -> f32 {
    a.w * b.w + a.x * b.x + a.y * b.y + a.z * b.z
}

//---------------------------------------------------------------------------
// slerp
//
// Spherical linear interpolation.
//
// See 10.4.13

pub fn slerp(q0: &Quaternion, q1: &Quaternion, t: f32) -> Quaternion {
    // Check for out-of range parameter and return edge points if so

    if t <= 0.0 {
        return q0.clone();
    }

    if t >= 1.0 {
        return q1.clone();
    }

    // Compute "cosine of angle between quaternions" using dot product

    let mut cos_omega = dot_product(q0, q1);

    // If negative dot, use -q1.  Two quaternions q and -q
    // represent the same rotation, but may produce
    // different slerp.  We chose q or -q to rotate using
    // the acute angle.

    let mut q1w = q1.w;
    let mut q1x = q1.x;
    let mut q1y = q1.y;
    let mut q1z = q1.z;

    if cos_omega < 0.0 {
        q1w = -q1w;
        q1x = -q1x;
        q1y = -q1y;
        q1z = -q1z;
        cos_omega = -cos_omega;
    }

    // We should have two unit quaternions, so dot should be <= 1.0

    assert!(cos_omega < 1.1);

    // Compute interpolation fraction, checking for quaternions
    // almost exactly the same

    let k0: f32;
    let k1: f32;

    if cos_omega > 0.9999 {
        // Very close - just use linear interpolation,
        // which will protect againt a divide by zero

        k0 = 1.0 - t;
        k1 = t;
    } else {
        // Compute the sin of the angle using the
        // trig identity sin^2(omega) + cos^2(omega) = 1

        let sin_omega = (1.0 - cos_omega * cos_omega).sqrt();

        // Compute the angle from its sin and cosine

        let omega = atan2(sin_omega, cos_omega);

        // Compute inverse of denominator, so we only have
        // to divide once

        let one_over_sin_omega = 1.0 / sin_omega;

        // Compute interpolation parameters

        k0 = ((1.0 - t) * omega).sin() * one_over_sin_omega;
        k1 = (t * omega).sin() * one_over_sin_omega;
    }

    // Interpolate

    Quaternion {
        x: k0 * q0.x + k1 * q1x,
        y: k0 * q0.y + k1 * q1y,
        z: k0 * q0.z + k1 * q1z,
        w: k0 * q0.w + k1 * q1w,
    }
}

//---------------------------------------------------------------------------
// conjugate
//
// Compute the quaternion conjugate.  This is the quaternian
// with the opposite rotation as the original quaternian.  See 10.4.7

pub fn conjugate(q: &Quaternion) -> Quaternion {
    Quaternion {
        // Same rotation amount
        w: q.w,
        // Opposite axis of rotation
        x: -q.x,
        y: -q.y,
        z: -q.z,
    }
}

//---------------------------------------------------------------------------
// pow
//
// Quaternion exponentiation.
pub fn pow(q: Quaternion, exponent: f32) -> Quaternion {
    // Check for the case of an identity quaternion.
    // This will protect against divide by zero

    if (q.w).abs() > 0.9999 {
        return q.clone();
    }

    // Extract the half angle alpha (alpha = theta/2)
    let alpha = (q.w).acos();

    // Compute new alpha value

    let new_alpha = alpha * exponent;

    // Compute new w value

    let mult = new_alpha.sin() / alpha.sin();

    Quaternion {
        w: new_alpha.cos(),
        // Compute new xyz values
        x: q.x * mult,
        y: q.y * mult,
        z: q.z * mult,
    }
}
