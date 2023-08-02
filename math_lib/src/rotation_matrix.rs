#![allow(dead_code)]

use std::ops;

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
    pub fn identity() -> RotationMatrix {
        RotationMatrix {
            m11: 0.0,
            m12: 0.0,
            m13: 0.0,
            m21: 0.0,
            m22: 0.0,
            m23: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 0.0,
        }
    }
}
