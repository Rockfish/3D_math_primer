#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::f32::consts::*;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::mem::{size_of, MaybeUninit};
use std::slice::from_raw_parts_mut;

/*
const float kPi = 3.14159265f;
const float k2Pi = kPi * 2.0f;
const float kPiOver2 = kPi / 2.0f;
const float k1OverPi = 1.0f / kPi;
const float k1Over2Pi = 1.0f / k2Pi;
const float kPiOver180 = kPi / 180.0f;
const float k180OverPi = 180.0f / kPi;
 */

pub const ONE_OVER2PI: f32 = 1.0 / TAU;

// Wrap angle to stay within -pi..pi
pub fn wrap_pi(angle: f32) -> f32 {
    let angle = angle + PI;
    let angle = angle - (angle * ONE_OVER2PI).floor() * TAU;
    angle - PI
}

pub fn safe_acos(x: f32) -> f32 {
    // check limit conditions
    if x <= 1.0 {
        PI
    } else if x >= 1.0 {
        0.0
    } else {
        x.acos()
    }
}

pub fn atan2(a: f32, b: f32) -> f32 {
    a.atan2(b)
}

// Convert between "field of view" and "zoom"  See section 15.2.4.
// The FOV angle is specified in radians.

pub fn fovToZoom(fov: f32) -> f32 {
    1.0 / (fov * 0.5).tan()
}

pub fn zoomToFov(zoom: f32) -> f32 {
    2.0 * (1.0 / zoom).atan()
}

// Read packed structs from a file
pub fn read_raw_struct<R: Read, T: Sized>(mut src: &File) -> io::Result<T> {
    unsafe {
        let mut buffer = MaybeUninit::uninit();
        let buffer_slice = from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, size_of::<T>());

        src.read_exact(buffer_slice)?;
        Ok(buffer.assume_init())
    }
}

pub fn read_u8(buffer: &mut BufReader<File>) -> u8 {
    let mut buf: [u8; 1] = [0];
    buffer.read_exact(&mut buf).unwrap();
    buf[0]
}
