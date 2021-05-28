use std::f64::consts::PI;

use crate::types::Vec3;

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        let tmp = x;
        x = y;
        y = tmp % y;
    }
    x
}

pub fn deg_to_radians(d: f64) -> f64 {
    (PI * d) / 180.
}

pub fn same_orientation(v1: &Vec3, v2: &Vec3) -> bool {
    dot(v1, v2) > 0.
}
