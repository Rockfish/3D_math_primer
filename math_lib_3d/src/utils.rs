#![allow(dead_code)]

use std::f32::consts::*;

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
