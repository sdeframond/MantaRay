use cgmath::ray::{Ray, Ray3};
use cgmath::vector::EuclideanVector;
use cgmath::point::Point;

use scene::Scene;
use light::Light;
use shape::Shape;
use material::Material;

pub fn trace_path(scene: &Scene, ray: Ray3<f32>) -> Light {
    match scene.intersect(ray) {
        None => scene.background(ray.direction),
        Some((object, shape, point)) => {
            let mut reflected = Light::zero();
            for source in scene.light_sources.iter() {
                let vec_to_light = source.origin().sub_p(&point);
                let unit_to_light = vec_to_light.normalize();
                let shadow_ray = Ray::new(point, unit_to_light);
                let shadowed = scene.shadow_intersect(shape, shadow_ray, vec_to_light.length());
                if !shadowed {
                    let reflectance = object.reflectance(point, -unit_to_light, -ray.direction);
                    reflected = reflected + source.intensity(point).mul_l(reflectance);
                }
            }
            reflected + object.emittance(point, -ray.direction)
        }
    }
}

#[cfg(test)]
mod tests {

    use cgmath::ray::Ray;
    use cgmath::vector::{EuclideanVector, Vector3};
    use cgmath::point::Point;

    use pathtracing::trace_path;
    use light::Light;
    use test_helpers::make_test_scene;

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