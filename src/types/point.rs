use crate::types::Vec3;
use std::ops::{Add, Sub};

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

macro_rules! vec_ops {
    ($point_t:ty, $vec_t:ty) => {
        impl Add<$vec_t> for $point_t {
            type Output = Point;

            fn add(self, rhs: $vec_t) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }
    };
}

vec_ops!(Point, Vec3);
vec_ops!(&Point, Vec3);
vec_ops!(Point, &Vec3);
vec_ops!(&Point, &Vec3);

macro_rules! points_ops {
    ($t1:ty, $t2:ty) => {
        impl Sub<$t2> for $t1 {
            type Output = Vec3;

            fn sub(self, rhs: $t2) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                }
            }
        }
    };
}

points_ops!(Point, Point);
points_ops!(&Point, Point);
points_ops!(Point, &Point);
points_ops!(&Point, &Point);
