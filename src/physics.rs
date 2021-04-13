use crate::math::dot;
use crate::types::Vec3;

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2. * dot(v, n) * n)
}
