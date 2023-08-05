pub mod aabb3;
pub mod euler_angles;
pub mod matrix4x3;
pub mod quaternion;
pub mod renderer;
pub mod rotation_matrix;
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
