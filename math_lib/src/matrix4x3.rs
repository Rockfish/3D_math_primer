#![allow(dead_code)]

use crate::vector::Vec3;

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
            m11: 0.0,
            m12: 0.0,
            m13: 0.0,
            m21: 0.0,
            m22: 0.0,
            m23: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 0.0,
            tx: 0.0,
            ty: 0.0,
            tz: 0.0,
        }
    }

    pub fn get_translation(&self) -> Vec3 {
        todo!();
    }
}

pub fn get_translation(_m: &Matrix4x3) -> Vec3 {
    todo!();
}
