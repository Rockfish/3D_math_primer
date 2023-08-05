#![allow(dead_code)]

use crate::euler_angles::EulerAngles;
use crate::quaternion::Quaternion;
use crate::rotation_matrix::RotationMatrix;
use crate::vector3::Vector3;
use std::ops;

/////////////////////////////////////////////////////////////////////////////
//
// Notes:
//
// See Chapter 11 for more information on class design decisions.
//
//---------------------------------------------------------------------------
//
// MATRIX ORGANIZATION
//
// The purpose of this class is so that a user might perform transformations
// without fiddling with plus or minus signs or transposing the matrix
// until the output "looks right."  But of course, the specifics of the
// internal representation is important.  Not only for the implementation
// in this file to be correct, but occasionally direct access to the
// matrix variables is necessary, or beneficial for optimization.  Thus,
// we document our matrix conventions here.
//
// We use row vectors, so multiplying by our matrix looks like this:
//
//               | m11 m12 m13 |
//     [ x y z ] | m21 m22 m23 | = [ x' y' z' ]
//               | m31 m32 m33 |
//               | tx  ty  tz  |
//
// Strict adherance to linear algebra rules dictates that this
// multiplication is actually undefined.  To circumvent this, we can
// consider the input and output vectors as having an assumed fourth
// coordinate of 1.  Also, since we cannot technically invert a 4x3 matrix
// according to linear algebra rules, we will also assume a rightmost
// column of [ 0 0 0 1 ].  This is shown below:
//
//                 | m11 m12 m13 0 |
//     [ x y z 1 ] | m21 m22 m23 0 | = [ x' y' z' 1 ]
//                 | m31 m32 m33 0 |
//                 | tx  ty  tz  1 |
//
// In case you have forgotten your linear algebra rules for multiplying
// matrices (which are described in section 7.1.6 and 7.1.7), see the
// definition of operator* for the expanded computations.
//
/////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Matrix4x3 {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub tx: f32,
    pub ty: f32,
    pub tz: f32,
}

impl Matrix4x3 {
    pub fn identity() -> Matrix4x3 {
        Matrix4x3 {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
            tx: 0.0,
            ty: 0.0,
            tz: 1.0,
        }
    }

    pub fn set_identity(&mut self) {
        self.m11 = 1.0;
        self.m12 = 0.0;
        self.m13 = 0.0;
        self.m21 = 0.0;
        self.m22 = 1.0;
        self.m23 = 0.0;
        self.m31 = 0.0;
        self.m32 = 0.0;
        self.m33 = 1.0;
        self.tx = 0.0;
        self.ty = 0.0;
        self.tz = 1.0;
    }

    //---------------------------------------------------------------------------
    // zero_translation
    //
    // Zero the 4th row of the matrix, which contains the translation portion.
    pub fn zero_translation(&mut self) {
        self.tx = 0.0;
        self.ty = 0.0;
        self.tz = 0.0;
    }

    //---------------------------------------------------------------------------
    // set_translation
    //
    // Sets the translation portion of the matrix in vector form
    pub fn set_translation(&mut self, d: &Vector3) {
        self.tx = d.x;
        self.ty = d.y;
        self.tz = d.z;
    }

    //---------------------------------------------------------------------------
    // setup_translation
    //
    // Sets the translation portion of the matrix in vector form
    pub fn setup_translation(&mut self, d: &Vector3) {
        // Set the linear transformation portion to identity
        self.m11 = 1.0;
        self.m12 = 0.0;
        self.m13 = 0.0;
        self.m21 = 0.0;
        self.m22 = 1.0;
        self.m23 = 0.0;
        self.m31 = 0.0;
        self.m32 = 0.0;
        self.m33 = 1.0;

        // Set the translation portion
        self.tx = d.x;
        self.ty = d.y;
        self.tz = d.z;
    }

    //---------------------------------------------------------------------------
    // setupLocalToParent
    //
    // Setup the matrix to perform a local -> parent transformation, given
    // the position and orientation of the local reference frame within the
    // parent reference frame.
    //
    // A very common use of this will be to construct a object -> world matrix.
    // As an example, the transformation in this case is straightforward.  We
    // first rotate from object space into inertial space, then we translate
    // into world space.
    //
    // We allow the orientation to be specified using either euler angles,
    // or a RotationMatrix
    pub fn setup_local_to_parent_euler_angles(&mut self, pos: &Vector3, orient: &EulerAngles) {
        // Create a rotation matrix.
        let orient_matrix = RotationMatrix::from_euler_angles(orient);

        // Setup the 4x3 matrix.  Note: if we were really concerned with
        // speed, we could create the matrix directly into these variables,
        // without using the temporary RotationMatrix object.  This would
        // save us a function call and a few copy operations.
        self.setup_local_to_parent_rotation_matrix(pos, &orient_matrix);
    }

    pub fn setup_local_to_parent_rotation_matrix(
        &mut self,
        pos: &Vector3,
        orient: &RotationMatrix,
    ) {
        // Copy the rotation portion of the matrix.  According to
        // the comments in RotationMatrix.cpp, the rotation matrix
        // is "normally" an inertial->object matrix, which is
        // parent->local.  We want a local->parent rotation, so we
        // must transpose while copying
        self.m11 = orient.m11;
        self.m12 = orient.m21;
        self.m13 = orient.m31;
        self.m21 = orient.m12;
        self.m22 = orient.m22;
        self.m23 = orient.m32;
        self.m31 = orient.m13;
        self.m32 = orient.m23;
        self.m33 = orient.m33;

        // Now set the translation portion.  Translation happens "after"
        // the 3x3 portion, so we can simply copy the position
        // field directly
        self.tx = pos.x;
        self.ty = pos.y;
        self.tz = pos.z;
    }

    //---------------------------------------------------------------------------
    // setupParentToLocal
    //
    // Setup the matrix to perform a parent -> local transformation, given
    // the position and orientation of the local reference frame within the
    // parent reference frame.
    //
    // A very common use of this will be to construct a world -> object matrix.
    // To perform this transformation, we would normally FIRST transform
    // from world to inertial space, and then rotate from inertial space into
    // object space.  However, out 4x3 matrix always translates last.  So
    // we think about creating two matrices T and R, and then concatenating
    // M = TR.
    //
    // We allow the orientation to be specified using either euler angles,
    // or a RotationMatrix
    pub fn setup_parent_to_local_euler_angles(&mut self, pos: &Vector3, orient: &EulerAngles) {
        // Create a rotation matrix.
        let orient_matrix = RotationMatrix::from_euler_angles(orient);

        // Setup the 4x3 matrix.
        self.setup_local_to_parent_rotation_matrix(pos, &orient_matrix);
    }

    pub fn setup_parent_to_local_rotation_matrix(
        &mut self,
        pos: &Vector3,
        orient: &RotationMatrix,
    ) {
        // Copy the rotation portion of the matrix.  We can copy the
        // elements directly (without transposing) according
        // to the layout as commented in RotationMatrix.cpp
        self.m11 = orient.m11;
        self.m12 = orient.m12;
        self.m13 = orient.m13;
        self.m21 = orient.m21;
        self.m22 = orient.m22;
        self.m23 = orient.m23;
        self.m31 = orient.m31;
        self.m32 = orient.m32;
        self.m33 = orient.m33;

        // Now set the translation portion.  Normally, we would
        // translate by the negative of the position to translate
        // from world to inertial space.  However, we must correct
        // for the fact that the rotation occurs "first."  So we
        // must rotate the translation portion.  This is the same
        // as create a translation matrix T to translate by -pos,
        // and a rotation matrix R, and then creating the matrix
        // as the concatenation of TR
        self.tx = -(pos.x * self.m11 + pos.y * self.m21 + pos.z * self.m31);
        self.ty = -(pos.x * self.m12 + pos.y * self.m22 + pos.z * self.m32);
        self.tz = -(pos.x * self.m13 + pos.y * self.m23 + pos.z * self.m33);
    }

    //---------------------------------------------------------------------------
    // setupRotate
    //
    // Setup the matrix to perform a rotation about a cardinal axis
    //
    // The axis of rotation is specified using a 1-based index:
    //
    //	1 => rotate about the x-axis
    //	2 => rotate about the y-axis
    //	3 => rotate about the z-axis
    //
    // theta is the amount of rotation, in radians.  The left-hand rule is
    // used to define "positive" rotation.
    //
    // The translation portion is reset.
    //
    // See 8.2.2 for more info.
    pub fn setup_rotate_axis(&mut self, axis: i32, theta: f32) {
        // Get sin and cosine of rotation angle
        let (sin, cos) = theta.sin_cos();

        // Check which axis they are rotating about
        match axis {
            // Rotate about the x-axis
            1 => {
                self.m11 = 1.0;
                self.m12 = 0.0;
                self.m13 = 0.0;
                self.m21 = 0.0;
                self.m22 = cos;
                self.m23 = sin;
                self.m31 = 0.0;
                self.m32 = -sin;
                self.m33 = cos;
            }
            2 => {
                // Rotate about the y-axis
                self.m11 = cos;
                self.m12 = 0.0;
                self.m13 = -sin;
                self.m21 = 0.0;
                self.m22 = 1.0;
                self.m23 = 0.0;
                self.m31 = sin;
                self.m32 = 0.0;
                self.m33 = cos;
            }
            3 => {
                // Rotate about the z-axis
                self.m11 = cos;
                self.m12 = sin;
                self.m13 = 0.0;
                self.m21 = -sin;
                self.m22 = cos;
                self.m23 = 0.0;
                self.m31 = 0.0;
                self.m32 = 0.0;
                self.m33 = 1.0;
            }
            _ => {
                // bogus axis index
                assert!(false);
            }
        }

        // Reset the translation portion
        self.tx = 0.0;
        self.ty = 0.0;
        self.tz = 0.0;
    }

    //---------------------------------------------------------------------------
    // setupRotate
    //
    // Setup the matrix to perform a rotation about an arbitrary axis.
    // The axis of rotation must pass through the origin.
    //
    // axis defines the axis of rotation, and must be a unit vector.
    //
    // theta is the amount of rotation, in radians.  The left-hand rule is
    // used to define "positive" rotation.
    //
    // The translation portion is reset.
    //
    // See 8.2.3 for more info.
    pub fn setup_rotate_from_vector(&mut self, axis: &Vector3, theta: f32) {
        // Quick sanity check to make sure they passed in a unit vector
        // to specify the axis
        assert!((axis.dot(axis) - 1.0).abs() < 0.01);

        // Get sin and cosine of rotation angle
        let (sin, cos) = theta.sin_cos();

        // Compute 1 - cos(theta) and some common subexpressions
        let a = 1.0 - cos;
        let ax = a * axis.x;
        let ay = a * axis.y;
        let az = a * axis.z;

        // Set the matrix elements.  There is still a little more
        // opportunity for optimization due to the many common
        // subexpressions.  We'll let the compiler handle that...
        self.m11 = ax * axis.x + cos;
        self.m12 = ax * axis.y + axis.z * sin;
        self.m13 = ax * axis.z - axis.y * sin;

        self.m21 = ay * axis.x - axis.z * sin;
        self.m22 = ay * axis.y + cos;
        self.m23 = ay * axis.z + axis.x * sin;

        self.m31 = az * axis.x + axis.y * sin;
        self.m32 = az * axis.y - axis.x * sin;
        self.m33 = az * axis.z + cos;

        // Reset the translation portion
        self.tx = 0.0;
        self.ty = 0.0;
        self.tz = 0.0;
    }

    //---------------------------------------------------------------------------
    // from_quaternion
    //
    // Setup the matrix to perform a rotation, given the angular displacement
    // in quaternion form.
    //
    // The translation portion is reset.
    //
    // See 10.6.3 for more info.
    pub fn set_from_quaternion(&mut self, q: &Quaternion) {
        // Compute a few values to optimize common subexpressions
        let ww = 2.0 * q.w;
        let xx = 2.0 * q.x;
        let yy = 2.0 * q.y;
        let zz = 2.0 * q.z;

        // Set the matrix elements.  There is still a little more
        // opportunity for optimization due to the many common
        // subexpressions.  We'll let the compiler handle that...
        self.m11 = 1.0 - yy * q.y - zz * q.z;
        self.m12 = xx * q.y + ww * q.z;
        self.m13 = xx * q.z - ww * q.x;

        self.m21 = xx * q.y - ww * q.z;
        self.m22 = 1.0 - xx * q.x - zz * q.z;
        self.m23 = yy * q.z + ww * q.x;

        self.m31 = xx * q.z + ww * q.y;
        self.m32 = yy * q.z - ww * q.x;
        self.m33 = 1.0 - xx * q.x - yy * q.y;

        // Reset the translation portion
        self.tx = 0.0;
        self.ty = 0.0;
        self.tz = 0.0;
    }

    //---------------------------------------------------------------------------
    // setup_scale
    //
    // Setup the matrix to perform scale on each axis.  For uniform scale by k,
    // use a vector of the form Vector3(k,k,k)
    //
    // The translation portion is reset.
    //
    // See 8.3.1 for more info.
    pub fn setup_scale(&mut self, s: &Vector3) {
        // Set the matrix elements.  Pretty straightforward
        self.m11 = s.x;
        self.m12 = 0.0;
        self.m13 = 0.0;
        self.m21 = 0.0;
        self.m22 = s.y;
        self.m23 = 0.0;
        self.m31 = 0.0;
        self.m32 = 0.0;
        self.m33 = s.z;

        // Reset the translation portion
        self.tx = 0.0;
        self.ty = 0.0;
        self.tz = 0.0;
    }

    //---------------------------------------------------------------------------
    // setup_scale_along_axis
    //
    // Setup the matrix to perform scale along an arbitrary axis.
    //
    // The axis is specified using a unit vector.
    //
    // The translation portion is reset.
    //
    // See 8.3.2 for more info.
    pub fn setup_scale_along_axis(&mut self, axis: &Vector3, k: f32) {
        // Quick sanity check to make sure they passed in a unit vector
        // to specify the axis
        assert!((axis.dot(axis) - 1.0).abs() < 0.01);

        // Compute k-1 and some common subexpressions
        let a = k - 1.0;
        let ax = a * axis.x;
        let ay = a * axis.y;
        let az = a * axis.z;

        // Fill in the matrix elements.  We'll do the common
        // subexpression optimization ourselves here, since diagonally
        // opposite matrix elements are equal
        self.m11 = ax * axis.x + 1.0;
        self.m22 = ay * axis.y + 1.0;
        self.m32 = az * axis.z + 1.0;

        self.m12 = ax * axis.y;
        self.m21 = ax * axis.y;
        self.m13 = ax * axis.z;
        self.m31 = ax * axis.z;
        self.m23 = ay * axis.z;
        self.m32 = ay * axis.z;

        // Reset the translation portion
        self.tx = 0.0;
        self.ty = 0.0;
        self.tz = 0.0;
    }

    //---------------------------------------------------------------------------
    // setup_shear
    //
    // Setup the matrix to perform a shear
    //
    // The type of shear is specified by the 1-based "axis" index.  The effect
    // of transforming a point by the matrix is described by the pseudocode
    // below:
    //
    //	axis == 1  =>  y += s*x, z += t*x
    //	axis == 2  =>  x += s*y, z += t*y
    //	axis == 3  =>  x += s*z, y += t*z
    //
    // The translation portion is reset.
    //
    // See 8.6 for more info.
    pub fn setup_shear(&mut self, axis: i32, s: f32, t: f32) {
        // Check which type of shear they want
        match axis {
            1 => {
                // Shear y and z using x
                self.m11 = 1.0;
                self.m12 = s;
                self.m13 = t;
                self.m21 = 0.0;
                self.m22 = 1.0;
                self.m23 = 0.0;
                self.m31 = 0.0;
                self.m32 = 0.0;
                self.m33 = 1.0;
            }
            2 => {
                // Shear x and z using y
                self.m11 = 1.0;
                self.m12 = 0.0;
                self.m13 = 0.0;
                self.m21 = s;
                self.m22 = 1.0;
                self.m23 = t;
                self.m31 = 0.0;
                self.m32 = 0.0;
                self.m33 = 1.0;
            }
            3 => {
                // Shear x and y using z
                self.m11 = 1.0;
                self.m12 = 0.0;
                self.m13 = 0.0;
                self.m21 = 0.0;
                self.m22 = 1.0;
                self.m23 = 0.0;
                self.m31 = s;
                self.m32 = t;
                self.m33 = 1.0;
            }
            _ => {
                // bogus axis index
                assert!(false);
            }
        }

        // Reset the translation portion
        self.tx = 0.0;
        self.ty = 0.0;
        self.tz = 0.0;
    }

    //---------------------------------------------------------------------------
    // setup_projection
    //
    // Setup the matrix to perform a projection onto a plane passing
    // through the origin.  The plane is perpendicular to the
    // unit vector n.
    //
    // See 8.4.2 for more info.
    pub fn setup_projection(&mut self, n: &Vector3) {
        // Quick sanity check to make sure they passed in a unit vector
        // to specify the axis
        assert!((n.dot(n) - 1.0).abs() < 0.01);

        // Fill in the matrix elements.  We'll do the common
        // subexpression optimization ourselves here, since diagonally
        // opposite matrix elements are equal
        self.m11 = 1.0 - n.x * n.x;
        self.m22 = 1.0 - n.y * n.y;
        self.m33 = 1.0 - n.z * n.z;

        self.m12 = -n.x * n.y;
        self.m21 = -n.x * n.y;
        self.m13 = -n.x * n.z;
        self.m31 = -n.x * n.z;
        self.m23 = -n.y * n.z;
        self.m32 = -n.y * n.z;

        // Reset the translation portion
        self.tx = 0.0;
        self.ty = 0.0;
        self.tz = 0.0;
    }

    //---------------------------------------------------------------------------
    // setupReflect
    //
    // Setup the matrix to perform a reflection about a plane parallel
    // to a cardinal plane.
    //
    // axis is a 1-based index which specifies the plane to project about:
    //
    //	1 => reflect about the plane x=k
    //	2 => reflect about the plane y=k
    //	3 => reflect about the plane z=k
    //
    // The translation is set appropriately, since translation must occur if
    // k != 0
    //
    // See 8.5 for more info.
    pub fn setup_reflection_from_axis(&mut self, axis: i32, k: f32) {
        // Check which plane they want to reflect about
        match axis {
            1 => {
                // Reflect about the plane x=k
                self.m11 = -1.0;
                self.m12 = 0.0;
                self.m13 = 0.0;
                self.m21 = 0.0;
                self.m22 = 1.0;
                self.m23 = 0.0;
                self.m31 = 0.0;
                self.m32 = 0.0;
                self.m33 = 1.0;

                self.tx = 2.0 * k;
                self.ty = 0.0;
                self.tz = 0.0;
            }
            2 => {
                // Reflect about the plane y=k
                self.m11 = 1.0;
                self.m12 = 0.0;
                self.m13 = 0.0;
                self.m21 = 0.0;
                self.m22 = -1.0;
                self.m23 = 0.0;
                self.m31 = 0.0;
                self.m32 = 0.0;
                self.m33 = 1.0;

                self.tx = 0.0;
                self.ty = 2.0 * k;
                self.tz = 0.0;
            }
            3 => {
                // Reflect about the plane z=k
                self.m11 = 1.0;
                self.m12 = 0.0;
                self.m13 = 0.0;
                self.m21 = 0.0;
                self.m22 = 1.0;
                self.m23 = 0.0;
                self.m31 = 0.0;
                self.m32 = 0.0;
                self.m33 = -1.0;

                self.tx = 0.0;
                self.ty = 0.0;
                self.tz = 2.0 * k;
            }
            _ => {
                // bogus axis index
                assert!(false);
            }
        }
    }

    //---------------------------------------------------------------------------
    // setupReflect
    //
    // Setup the matrix to perform a reflection about an arbitrary plane
    // through the origin.  The unit vector n is perpendicular to the plane.
    //
    // The translation portion is reset.
    //
    // See 8.5 for more info.
    pub fn setup_reflection_from_vector(&mut self, n: &Vector3) {
        // Quick sanity check to make sure they passed in a unit vector
        // to specify the axis
        assert!((n.dot(n) - 1.0).abs() < 0.01);

        // Compute common subexpressions
        let ax = -2.0 * n.x;
        let ay = -2.0 * n.y;
        let az = -2.0 * n.z;

        // Fill in the matrix elements.  We'll do the common
        // subexpression optimization ourselves here, since diagonally
        // opposite matrix elements are equal
        self.m11 = 1.0 + ax * n.x;
        self.m22 = 1.0 + ay * n.y;
        self.m32 = 1.0 + az * n.z;

        self.m12 = ax * n.y;
        self.m21 = ax * n.y;
        self.m13 = ax * n.z;
        self.m31 = ax * n.z;
        self.m23 = ay * n.z;
        self.m32 = ay * n.z;

        // Reset the translation portion
        self.tx = 0.0;
        self.ty = 0.0;
        self.tz = 0.0;
    }
}

//---------------------------------------------------------------------------
// Vector * Matrix4x3
//
// Transform the point.  This makes using the vector class look like it
// does with linear algebra notation on paper.
//
// We also provide a *= operator, as per C convention.
//
// See 7.1.7
impl ops::Mul<&Matrix4x3> for Vector3 {
    type Output = Vector3;

    fn mul(self, m: &Matrix4x3) -> Self::Output {
        Vector3 {
            x: self.x * m.m11 + self.y * m.m21 + self.z * m.m31 + m.tx,
            y: self.x * m.m12 + self.y * m.m22 + self.z * m.m32 + m.ty,
            z: self.x * m.m13 + self.y * m.m23 + self.z * m.m33 + m.tz,
        }
    }
}

//---------------------------------------------------------------------------
//  Vector *= Matrix4x3
//
impl ops::MulAssign<Matrix4x3> for Vector3 {
    fn mul_assign(&mut self, m: Matrix4x3) {
        self.x = self.x * m.m11 + self.y * m.m21 + self.z * m.m31 + m.tx;
        self.y = self.x * m.m12 + self.y * m.m22 + self.z * m.m32 + m.ty;
        self.z = self.x * m.m13 + self.y * m.m23 + self.z * m.m33 + m.tz;
    }
}

//---------------------------------------------------------------------------
// Matrix4x3 * Matrix4x3
//
// Matrix concatenation.  This makes using the vector class look like it
// does with linear algebra notation on paper.
//
// We also provide a *= operator, as per C convention.
//
// See 7.1.6

impl ops::Mul for Matrix4x3 {
    type Output = Matrix4x3;

    fn mul(self, b: Self) -> Self::Output {
        Matrix4x3 {
            // Compute the upper 3x3 (linear transformation) portion
            m11: self.m11 * b.m11 + self.m12 * b.m21 + self.m13 * b.m31,
            m12: self.m11 * b.m12 + self.m12 * b.m22 + self.m13 * b.m32,
            m13: self.m11 * b.m13 + self.m12 * b.m23 + self.m13 * b.m33,

            m21: self.m21 * b.m11 + self.m22 * b.m21 + self.m23 * b.m31,
            m22: self.m21 * b.m12 + self.m22 * b.m22 + self.m23 * b.m32,
            m23: self.m21 * b.m13 + self.m22 * b.m23 + self.m23 * b.m33,

            m31: self.m31 * b.m11 + self.m32 * b.m21 + self.m33 * b.m31,
            m32: self.m31 * b.m12 + self.m32 * b.m22 + self.m33 * b.m32,
            m33: self.m31 * b.m13 + self.m32 * b.m23 + self.m33 * b.m33,

            // Compute the translation portion
            tx: self.tx * b.m11 + self.ty * b.m21 + self.tz * b.m31 + b.tx,
            ty: self.tx * b.m12 + self.ty * b.m22 + self.tz * b.m32 + b.ty,
            tz: self.tx * b.m13 + self.ty * b.m23 + self.tz * b.m33 + b.tz,
        }
    }
}

impl ops::MulAssign for Matrix4x3 {
    fn mul_assign(&mut self, b: Self) {
        // Compute the upper 3x3 (linear transformation) portion
        self.m11 = self.m11 * b.m11 + self.m12 * b.m21 + self.m13 * b.m31;
        self.m12 = self.m11 * b.m12 + self.m12 * b.m22 + self.m13 * b.m32;
        self.m13 = self.m11 * b.m13 + self.m12 * b.m23 + self.m13 * b.m33;

        self.m21 = self.m21 * b.m11 + self.m22 * b.m21 + self.m23 * b.m31;
        self.m22 = self.m21 * b.m12 + self.m22 * b.m22 + self.m23 * b.m32;
        self.m23 = self.m21 * b.m13 + self.m22 * b.m23 + self.m23 * b.m33;

        self.m31 = self.m31 * b.m11 + self.m32 * b.m21 + self.m33 * b.m31;
        self.m32 = self.m31 * b.m12 + self.m32 * b.m22 + self.m33 * b.m32;
        self.m33 = self.m31 * b.m13 + self.m32 * b.m23 + self.m33 * b.m33;

        // Compute the translation portion
        self.tx = self.tx * b.m11 + self.ty * b.m21 + self.tz * b.m31 + b.tx;
        self.ty = self.tx * b.m12 + self.ty * b.m22 + self.tz * b.m32 + b.ty;
        self.tz = self.tx * b.m13 + self.ty * b.m23 + self.tz * b.m33 + b.tz;
    }
}

//---------------------------------------------------------------------------
// determinant
//
// Compute the determinant of the 3x3 portion of the matrix.
//
// See 9.1.1 for more info.
pub fn determinant(m: &Matrix4x3) -> f32 {
    m.m11 * (m.m22 * m.m33 - m.m23 * m.m32)
        + m.m12 * (m.m23 * m.m31 - m.m21 * m.m33)
        + m.m13 * (m.m21 * m.m32 - m.m22 * m.m31)
}

//---------------------------------------------------------------------------
// inverse
//
// Compute the inverse of a matrix.  We use the classical adjoint divided
// by the determinant method.
//
// See 9.2.1 for more info.
pub fn inverse(m: &Matrix4x3) -> Matrix4x3 {
    // Compute the determinant
    let det = determinant(m);

    // If we're singular, then the determinant is zero and there's
    // no inverse
    assert!((det).abs() > 0.000001);

    // Compute one over the determinant, so we divide once and
    // can *multiply* per element
    let one_over_det = 1.0 / det;

    let mut r = Matrix4x3::identity();
    // Compute the 3x3 portion of the inverse, by
    // dividing the adjoint by the determinant
    r.m11 = (m.m22 * m.m33 - m.m23 * m.m32) * one_over_det;
    r.m12 = (m.m13 * m.m32 - m.m12 * m.m33) * one_over_det;
    r.m13 = (m.m12 * m.m23 - m.m13 * m.m22) * one_over_det;

    r.m21 = (m.m23 * m.m31 - m.m21 * m.m33) * one_over_det;
    r.m22 = (m.m11 * m.m33 - m.m13 * m.m31) * one_over_det;
    r.m23 = (m.m13 * m.m21 - m.m11 * m.m23) * one_over_det;

    r.m31 = (m.m21 * m.m32 - m.m22 * m.m31) * one_over_det;
    r.m32 = (m.m12 * m.m31 - m.m11 * m.m32) * one_over_det;
    r.m33 = (m.m11 * m.m22 - m.m12 * m.m21) * one_over_det;

    // Compute the translation portion of the inverse
    r.tx = -(m.tx * r.m11 + m.ty * r.m21 + m.tz * r.m31);
    r.ty = -(m.tx * r.m12 + m.ty * r.m22 + m.tz * r.m32);
    r.tz = -(m.tx * r.m13 + m.ty * r.m23 + m.tz * r.m33);

    // Return it.
    r
}

//---------------------------------------------------------------------------
// get_translation
//
// Return the translation row of the matrix in vector form
pub fn get_translation(m: &Matrix4x3) -> Vector3 {
    Vector3 {
        x: m.tx,
        y: m.ty,
        z: m.tz,
    }
}

//---------------------------------------------------------------------------
// get_position_from_parent_to_local_matrix
//
// Extract the position of an object given a parent -> local transformation
// matrix (such as a world -> object matrix)
//
// We assume that the matrix represents a rigid transformation.  (No scale,
// skew, or mirroring)
pub fn get_position_from_parent_to_local_matrix(m: &Matrix4x3) -> Vector3 {
    // Multiply negative translation value by the
    // transpose of the 3x3 portion.  By using the transpose,
    // we assume that the matrix is orthogonal.  (This function
    // doesn't really make sense for non-rigid transformations...)
    Vector3 {
        x: -(m.tx * m.m11 + m.ty * m.m12 + m.tz * m.m13),
        y: -(m.tx * m.m21 + m.ty * m.m22 + m.tz * m.m23),
        z: -(m.tx * m.m31 + m.ty * m.m32 + m.tz * m.m33),
    }
}

//---------------------------------------------------------------------------
// get_position_from_local_to_parent_matrix
//
// Extract the position of an object given a local -> parent transformation
// matrix (such as an object -> world matrix)
pub fn get_position_from_local_to_parent_matrix(m: &Matrix4x3) -> Vector3 {
    // Position is simply the translation portion
    Vector3 {
        x: m.tx,
        y: m.ty,
        z: m.tz,
    }
}
