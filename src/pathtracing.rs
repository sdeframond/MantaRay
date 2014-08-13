use cgmath::ray::Ray3;
use cgmath::vector::Vector3;

use scene::*;
use light::*;
use shape::*;
use material::*;

pub fn trace_path(scene: &Scene, ray: Ray3<f32>) -> Light {
    match scene.intersect(ray) {
        None => scene.background(ray.direction),
        Some((object, point)) => {
            let reflected = Light::new(1.0, 1.0, 1.0).mul_l(object.reflectance(point, Vector3::new(1.0, 0.0, 0.0), -ray.direction));
            object.emittance(point).add(reflected)
        }
    }
}

#[cfg(test)]
mod tests {

    use cgmath::ray::Ray;
    use cgmath::vector::{EuclideanVector, Vector3};
    use cgmath::point::Point;

    use pathtracing::*;
    use light::*;
    use test_helpers::*;

    macro_rules! assert_tp(
        ($scene:expr, $ray:expr, $r:expr, $g:expr, $b:expr) => (
            assert!(Light::new($r, $g, $b) == trace_path($scene, $ray))
        );
    )

    #[test]
    fn test_trace_path() {
        let delta = 0.000001f32;
        let scene = make_test_scene();
        let origin = Point::origin();
        let ray_miss = Ray::new(origin, Vector3::new(12.0/5.0 + delta, 0.0, 16.0/5.0).normalize());
        assert_tp!(&scene, ray_miss, 0.0, 0.0, 0.0);
        let ray_hit = Ray::new(origin, Vector3::new(12.0/5.0, 0.0, 16.0/5.0).normalize());
        assert_tp!(&scene, ray_hit, 1.0, 1.0, 1.0);
        let ray_miss2 = Ray::new(origin, Vector3::new(-12.0/5.0 - delta, 0.0, 16.0/5.0).normalize());
        assert_tp!(&scene, ray_miss2, 0.0, 0.0, 0.0);
        let ray_hit2 = Ray::new(origin, Vector3::new(-12.0/5.0, 0.0, 16.0/5.0).normalize());
        assert_tp!(&scene, ray_hit2, 1.0, 1.0, 1.0);
    }
}