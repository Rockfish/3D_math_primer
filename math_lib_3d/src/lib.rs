//#![feature(drain_filter)]
#![feature(extract_if)]

pub mod aabb3;
pub mod bitmap;
pub mod config;
pub mod edit_tri_mesh;
pub mod euler_angles;
pub mod matrix4x3;
pub mod model;
pub mod quaternion;
pub mod renderer;
pub mod rotation_matrix;
pub mod s3d_handler;
pub mod tri_mesh;
pub mod utils;
pub mod vector3;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
