#![allow(dead_code)]

// Rotation Matrix
//
// MATRIX ORGANIZATION
//
// A user of this class should rarely care how the matrix is organized.
// However, it is of course important that internally we keep everything
// straight.
//
// The matrix is assumed to be a rotation matrix only, and therefore
// orthoganal.  The "forward" direction of transformation (if that really
// even applies in this case) will be from inertial to object space.
// To perform an object->inertial rotation, we will multiply by the
// transpose.
//
// In other words:
//
// Inertial to object:
//
//                  | m11 m12 m13 |
//     [ ix iy iz ] | m21 m22 m23 | = [ ox oy oz ]
//                  | m31 m32 m33 |
//
// Object to inertial:
//
//                  | m11 m21 m31 |
//     [ ox oy oz ] | m12 m22 m32 | = [ ix iy iz ]
//                  | m13 m23 m33 |
//
// Or, using column vector notation:
//
// Inertial to object:
//
//     | m11 m21 m31 | | ix |	| ox |
//     | m12 m22 m32 | | iy | = | oy |
//     | m13 m23 m33 | | iz |	| oz |
//
// Object to inertial:
//
//     | m11 m12 m13 | | ox |	| ix |
//     | m21 m22 m23 | | oy | = | iy |
//     | m31 m32 m33 | | oz |	| iz |
//
/////////////////////////////////////////////////////////////////////////////

use crate::euler_angles::EulerAngles;
use crate::quaternion::Quaternion;
use crate::vector3::Vector3;

#[derive(Debug)]
pub struct RotationMatrix {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
}

impl RotationMatrix {
    // Set the matrix to the identity matrix
    pub fn identity() -> RotationMatrix {
        RotationMatrix {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
        }
    }

    // Setup the matrix with the specified orientation
    pub fn setup(&mut self, orientation: &EulerAngles) {
        // Fetch sine and cosine of angles

        let (sin_heading, cos_heading) = orientation.heading.sin_cos();
        let (sin_pitch, cos_pitch) = orientation.pitch.sin_cos();
        let (sin_bank, cos_bank) = orientation.bank.sin_cos();

        // Fill in the matrix elements

        self.m11 = cos_heading * cos_bank + sin_heading * sin_pitch * sin_bank;
        self.m12 = -cos_heading * sin_bank + sin_heading * sin_pitch * cos_bank;
        self.m13 = sin_heading * cos_pitch;

        self.m21 = sin_bank * cos_pitch;
        self.m22 = cos_bank * cos_pitch;
        self.m23 = -sin_pitch;

        self.m31 = -sin_heading * cos_bank + cos_heading * sin_pitch * sin_bank;
        self.m32 = sin_bank * sin_heading + cos_heading * sin_pitch * cos_bank;
        self.m33 = cos_heading * cos_pitch;
    }

    // Setup new matrix with the specified orientation
    pub fn from_euler_angles(orientation: &EulerAngles) -> RotationMatrix {
        // Fetch sine and cosine of angles

        let (sin_heading, cos_heading) = orientation.heading.sin_cos();
        let (sin_pitch, cos_pitch) = orientation.pitch.sin_cos();
        let (sin_bank, cos_bank) = orientation.bank.sin_cos();

        // Fill in the matrix elements

        RotationMatrix {
            m11: cos_heading * cos_bank + sin_heading * sin_pitch * sin_bank,
            m12: -cos_heading * sin_bank + sin_heading * sin_pitch * cos_bank,
            m13: sin_heading * cos_pitch,

            m21: sin_bank * cos_pitch,
            m22: cos_bank * cos_pitch,
            m23: -sin_pitch,

            m31: -sin_heading * cos_bank + cos_heading * sin_pitch * sin_bank,
            m32: sin_bank * sin_heading + cos_heading * sin_pitch * cos_bank,
            m33: cos_heading * cos_pitch,
        }
    }

    // Setup the matrix, given a quaternion that performs an inertial->object
    // rotation
    pub fn set_from_inertial_to_object_quaternion(&mut self, q: &Quaternion) {
        // Fill in the matrix elements.  This could possibly be
        // optimized since there are many common subexpressions.
        // We'll leave that up to the compiler...

        self.m11 = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);
        self.m12 = 2.0 * (q.x * q.y + q.w * q.z);
        self.m13 = 2.0 * (q.x * q.z - q.w * q.y);

        self.m21 = 2.0 * (q.x * q.y - q.w * q.z);
        self.m22 = 1.0 - 2.0 * (q.x * q.x + q.z * q.z);
        self.m23 = 2.0 * (q.y * q.z + q.w * q.x);

        self.m31 = 2.0 * (q.x * q.z + q.w * q.y);
        self.m32 = 2.0 * (q.y * q.z - q.w * q.x);
        self.m33 = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);
    }

    // Setup the matrix, given a quaternion that performs an object->inertial
    // rotation
    pub fn set_from_object_to_inertial_quaternion(&mut self, q: &Quaternion) {
        // Fill in the matrix elements.  This could possibly be
        // optimized since there are many common subexpressions.
        self.m11 = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);
        self.m12 = 2.0 * (q.x * q.y - q.w * q.z);
        self.m13 = 2.0 * (q.x * q.z + q.w * q.y);

        self.m21 = 2.0 * (q.x * q.y + q.w * q.z);
        self.m22 = 1.0 - 2.0 * (q.x * q.x + q.z * q.z);
        self.m23 = 2.0 * (q.y * q.z - q.w * q.x);

        self.m31 = 2.0 * (q.x * q.z - q.w * q.y);
        self.m32 = 2.0 * (q.y * q.z + q.w * q.x);
        self.m33 = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);
    }

    // Rotate a vector from inertial to object space
    pub fn inertial_to_object(&self, v: &Vector3) -> Vector3 {
        // Perform the matrix multiplication in the "standard" way.
        Vector3 {
            x: self.m11 * v.x + self.m21 * v.y + self.m31 * v.z,
            y: self.m12 * v.x + self.m22 * v.y + self.m32 * v.z,
            z: self.m13 * v.x + self.m23 * v.y + self.m33 * v.z,
        }
    }

    // Rotate a vector from object to inertial space
    pub fn object_to_inertial(&self, v: &Vector3) -> Vector3 {
        // Multiply by the transpose
        Vector3 {
            x: self.m11 * v.x + self.m12 * v.y + self.m13 * v.z,
            y: self.m21 * v.x + self.m22 * v.y + self.m23 * v.z,
            z: self.m31 * v.x + self.m32 * v.y + self.m33 * v.z,
        }
    }
}
