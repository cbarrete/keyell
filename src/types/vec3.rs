use rand::{rngs::SmallRng, Rng};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn unit(&self) -> UnitVec3 {
        UnitVec3(self / self.len())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UnitVec3(Vec3);

impl UnitVec3 {
    pub fn unchecked_from(v: &Vec3) -> Self {
        Self(v.clone())
    }

    pub fn get(&self) -> &Vec3 {
        &self.0
    }

    pub fn random(rng: &mut SmallRng) -> Self {
        let random_vector = Vec3 {
            x: rng.gen_range(-1. ..1.),
            y: rng.gen_range(-1. ..1.),
            z: rng.gen_range(-1. ..1.),
        };
        random_vector.unit()
    }
}

impl Neg for &UnitVec3 {
    type Output = UnitVec3;

    fn neg(self) -> Self::Output {
        UnitVec3(-self.0.clone())
    }
}

macro_rules! vecs_ops {
    ($t1:ty, $t2:ty) => {
        impl Add<$t2> for $t1 {
            type Output = Vec3;

            fn add(self, rhs: $t2) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }

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

vecs_ops!(Vec3, Vec3);
vecs_ops!(&Vec3, Vec3);
vecs_ops!(Vec3, &Vec3);
vecs_ops!(&Vec3, &Vec3);

macro_rules! f32_ops {
    ($t:ty) => {
        impl Mul<$t> for f32 {
            type Output = Vec3;

            fn mul(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self * rhs.x,
                    y: self * rhs.y,
                    z: self * rhs.z,
                }
            }
        }

        impl Div<f32> for $t {
            type Output = Vec3;

            fn div(self, rhs: f32) -> Self::Output {
                (1. / rhs) * self
            }
        }

        impl Neg for $t {
            type Output = Vec3;

            fn neg(self) -> Self::Output {
                -1. * self
            }
        }
    };
}

f32_ops!(Vec3);
f32_ops!(&Vec3);
