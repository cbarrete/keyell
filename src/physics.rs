use crate::math::dot;
use crate::types::Vec3;

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - (2. * dot(v, n) * n)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3::new(x, y, z)
    }

    #[test]
    fn reflection_tests() {
        let n = v(0., 0., 1.);
        assert_eq!(reflect(&v(0., 0., 1.), &n), v(0., 0., -1.));
        assert_eq!(reflect(&v(0., 1., 1.), &n), v(0., 1., -1.));
        assert_eq!(reflect(&v(1., 0., -1.), &n), v(1., 0., 1.));
        assert_eq!(reflect(&v(0., 1., 0.), &n), v(0., 1., 0.));
    }
}
