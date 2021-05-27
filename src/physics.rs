use crate::math::dot;
use crate::types::Vec3;

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - (2. * dot(v, n) * n)
}

// TODO document and test
pub fn refract(v: &Vec3, n: &Vec3, n_ratio: f64) -> Vec3 {
    let cos_theta = dot(&-v, n);
    let out_perp = n_ratio * (v + cos_theta * n);
    let len_squared = out_perp.x.powi(2) + out_perp.y.powi(2) + out_perp.z.powi(2);
    let out_parallel = -(1.0 - len_squared).abs().sqrt() * n;
    out_perp + out_parallel
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
        assert_eq!(reflect(&v(0., 0., 0.), &n), v(0., 0., 0.));
    }
}
