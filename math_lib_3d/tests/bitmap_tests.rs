use math_lib_3d;
use math_lib_3d::bitmap::*;
use math_lib_3d::utils::read_raw_struct;
use std::fs::File;

#[test]
fn test_read_bitmap_header() {
    let filename = "/Users/john/Dev_Rust/Dev/math_lib_3d/viewer/AR_COUCH.tga";

    let file = File::open(filename).unwrap();
    let r = read_raw_struct::<File, TGAHeader>(&file);

    match r {
        Ok(header) => {
            println!("TGAHeader: {:?}", header);
        }
        Err(message) => {
            println!("Error: {:?}", message);
        }
    }
}

#[test]
fn test_read_tga() {
    let mut bitmap = Bitmap::default();
    let filename = "/Users/john/Dev_Rust/Dev/math_lib_3d/viewer/AR_COUCH.tga";

    let result = bitmap.loadTGA(filename);

    println!("result: {:?}", result);
    println!("bitmap: {:?}", bitmap);
}
