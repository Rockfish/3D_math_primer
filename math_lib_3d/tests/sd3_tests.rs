use math_lib_3d;
use math_lib_3d::edit_tri_mesh::*;
use math_lib_3d::s3d_handler::*;

#[test]
fn test_read_s3d_file() {
    let result = import_s3d("/Users/john/Dev_Rust/Dev/math_lib_3d/viewer/AR_COUCH.s3d");

    println!("result: {:?}", result);
    println!("\ndone.")
}
