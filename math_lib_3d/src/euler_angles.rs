#![allow(dead_code)]

use crate::matrix4x3::Matrix4x3;
use crate::quaternion::Quaternion;
use crate::rotation_matrix::RotationMatrix;
use crate::utils::*;
use std::f32::consts::*;

/// Represents a heading-pitch-bank Euler angle triangle
#[derive(Clone, Debug)]
pub struct EulerAngles {
    pub heading: f32,
    pub pitch: f32,
    pub bank: f32,
}

impl EulerAngles {
    pub fn identity() -> EulerAngles {
        EulerAngles {
            heading: 0.0,
            pitch: 0.0,
            bank: 0.0,
        }
    }

    // Determine "canonical" Euler angle triple
    // Set the Euler angle triple to its "canonical" value. This does not change
    // the meaning of the Euler angles as a representation of Orientation in 3D,
    // but if the angles are for other purposes such as angular velocities, etc,
    // then the operation might not be valid.
    pub fn canonize(&mut self) {
        self.pitch = wrap_pi(self.pitch);

        // Now, check for "the back side" of the matrix, pitch outside
        // the canonical range of -pi/2 ... pi/2
        if self.pitch < -FRAC_2_PI {
            self.pitch = -PI - self.pitch;
            self.heading += PI;
            self.bank += PI;
        } else if self.pitch > FRAC_2_PI {
            self.pitch = PI - self.pitch;
            self.heading += PI;
            self.bank += PI;
        }

        // OK, now check for the gimbal lock case (within a slight
        // tolerance)
        if (self.pitch).abs() > FRAC_2_PI - 1e-4 {
            // We are in gimbal lock. Assign all rotation
            // about the vertical axis to heading
            self.heading += self.bank;
            self.bank = 0.0;
        } else {
            // Not in gimbal lock. Wrap the bank angle in
            // canonical range
            self.bank = wrap_pi(self.bank);
        }

        // Wrap heading in canonical range
        self.heading = wrap_pi(self.heading);
    }

    // Setup the Euler angles, given an object->inertial rotation quaternion
    pub fn from_object_to_inertial_quaternion(q: &Quaternion) -> EulerAngles {
        // Extract sin(pitch)
        let sp = -2.0 * (q.y * q.z - q.w * q.x);

        // Check for Gimbal lock, giving slight tolerance for numerical imprecision
        if sp.abs() > 0.9999 {
            EulerAngles {
                // Looking straight up or down
                pitch: FRAC_2_PI * sp,
                // Compute heading, slam bank to zero
                heading: atan2(-q.x * q.z + q.w * q.y, 0.5 - q.y * q.y - q.z * q.z),
                bank: 0.0,
            }
        } else {
            // Compute angles.  We don't have to use the "safe" asin
            // function because we already checked for range errors when
            // checking for Gimbal lock
            EulerAngles {
                pitch: sp.asin(),
                heading: atan2(q.x * q.z + q.w * q.y, 0.5 - q.x * q.x - q.y * q.y),
                bank: atan2(q.x * q.y + q.w * q.z, 0.5 - q.x * q.x - q.z * q.z).atan(),
            }
        }
    }

    // Setup the Euler angles, given an inertial->object rotation quaternion
    pub fn from_inertial_to_object_quaternion(q: &Quaternion) -> EulerAngles {
        // Extract sin(pitch)
        let sp = -2.0 * (q.y * q.z + q.w * q.x);

        // Check for Gimbal lock, giving slight tolerance for numerical imprecision
        if sp.abs() > 0.9999 {
            EulerAngles {
                // Looking straight up or down
                pitch: FRAC_2_PI * sp,
                // Compute heading, slam bank to zero
                heading: atan2(-q.x * q.z - q.w * q.y, 0.5 - q.y * q.y - q.z * q.z),
                bank: 0.0,
            }
        } else {
            // Compute angles.  We don't have to use the "safe" asin
            // function because we already checked for range errors when
            // checking for Gimbal lock
            EulerAngles {
                pitch: sp.asin(),
                heading: atan2(q.x * q.z - q.w * q.y, 0.5 - q.x * q.x - q.y * q.y),
                bank: atan2(q.x * q.y - q.w * q.z, 0.5 - q.x * q.x - q.z * q.z),
            }
        }
    }

    // Setup the Euler angles, given a world->object transformation matrix.
    // The matrix is assumed to be orthogonal. The translation portion is ignored.
    pub fn from_world_to_object_matrix(m: &Matrix4x3) -> EulerAngles {
        // Extract sin(pitch) from m23.

        let sp = -m.m23;

        // Check for Gimbal lock
        if sp.abs() > 9.99999 {
            EulerAngles {
                // Looking straight up or down
                pitch: FRAC_2_PI * sp,
                // Compute heading, slam bank to zero
                heading: atan2(-m.m31, m.m11),
                bank: 0.0,
            }
        } else {
            // Compute angles.  We don't have to use the "safe" asin
            // function because we already checked for range errors when
            // checking for Gimbal lock

            EulerAngles {
                heading: atan2(m.m13, m.m33).atan(),
                pitch: sp.asin(),
                bank: atan2(m.m21, m.m22),
            }
        }
    }

    // Setup the Euler angles, given a rotation matrix.
    pub fn from_rotation_matrix(m: &RotationMatrix) -> EulerAngles {
        // Extract sin(pitch) from m23.
        let sp = -m.m23;

        // Check for Gimbal lock
        if sp.abs() > 9.99999 {
            EulerAngles {
                // Looking straight up or down
                pitch: FRAC_2_PI * sp,
                // Compute heading, slam bank to zero
                heading: atan2(-m.m31, m.m11),
                bank: 0.0,
            }
        } else {
            // Compute angles.  We don't have to use the "safe" asin
            // function because we already checked for range errors when
            // checking for Gimbal lock
            EulerAngles {
                heading: atan2(m.m13, m.m33),
                pitch: sp.asin(),
                bank: atan2(m.m21, m.m22),
            }
        }
    }
}
