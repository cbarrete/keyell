use rand::{thread_rng, Rng};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone)]
pub struct UnitVec3(Vec3);

impl UnitVec3 {
    pub fn unchecked_from(v: &Vec3) -> Self {
        Self(v.clone())
    }

    pub fn get(&self) -> &Vec3 {
        &self.0
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        let random_vector = Vec3 {
            x: rng.gen_range(-1., 1.),
            y: rng.gen_range(-1., 1.),
            z: rng.gen_range(-1., 1.),
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

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn unit(&self) -> UnitVec3 {
        UnitVec3(self / self.len())
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

macro_rules! f64_ops {
    ($t:ty) => {
        impl Mul<$t> for f64 {
            type Output = Vec3;

            fn mul(self, rhs: $t) -> Self::Output {
                Self::Output {
                    x: self * rhs.x,
                    y: self * rhs.y,
                    z: self * rhs.z,
                }
            }
        }

        impl Div<f64> for $t {
            type Output = Vec3;

            fn div(self, rhs: f64) -> Self::Output {
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

f64_ops!(Vec3);
f64_ops!(&Vec3);
