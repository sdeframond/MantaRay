use cgmath::vector::{EuclideanVector, Vector3};
use cgmath::ray::Ray3;
use cgmath::point::{Point, Point3};

use shape::Shape;
use object::Object;
use light::{Light, LightSource};
use utils::cmp_float;

pub struct Scene {
    pub objects: Vec<Object>,
    pub light_sources: Vec<Box<LightSource>>
}

impl Scene {
    pub fn background(&self, _direction: Vector3<f32>) -> Light {
        Light::new(0.0, 0.0, 0.0)
    }

    pub fn intersect(&self, ray: Ray3<f32>) -> Option<(&Object, Point3<f32>)> {
        let mut intersections: Vec<(&Object, Point3<f32>)> = self.objects.iter()
            .map(|obj| obj.intersect(ray))
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();
        let distance_cmp = |v1: &(&Object, Point3<f32>), v2: &(&Object, Point3<f32>)| {
            cmp_float(
                ray.origin.sub_p(v2.ref1()).length(),
                ray.origin.sub_p(v1.ref1()).length()
            )
        };
        intersections.sort_by(distance_cmp);
        intersections.pop()
    }
}

#[cfg(test)]
mod tests {

    use cgmath::vector::{EuclideanVector, Vector3};
    use cgmath::ray::{Ray, Ray3};
    use cgmath::point::{Point, Point3};

    use shape::{Plane, Sphere};
    use scene::Scene;
    use object::Object;
    use material::TestMaterial;

    fn get_point(scene: &Scene, ray: Ray3<f32>) -> Point3<f32> {
        scene.intersect(ray).unwrap().val1()
    }

    #[test]
    fn test_scene_intersect_hit() {
        let delta = 0.000001;
        let sphere = box Sphere {center: Point3::new(0.0f32, 0.0, 5.0), radius: 4.0};
        let obj = Object {shape: sphere, material: box TestMaterial};
        let scene = Scene {objects: vec![obj], light_sources: vec![]};
        let ray_hit = Ray::new(Point::origin(), Vector3::new(0.0, 0.0, 1.0));
        assert!(Point3::new(0.0, 0.0, 1.0) == get_point(&scene, ray_hit));
        let ray_miss = Ray::new(Point3::new(10.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert!(scene.intersect(ray_miss).is_none());
        let ray_border1 = Ray::new(Point3::new(4.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert!(Point3::new(4.0, 0.0, 5.0) == get_point(&scene, ray_border1));
        let ray_near1 = Ray::new(Point3::new(4.0 + delta, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert!(scene.intersect(ray_near1).is_none());
        let ray_near_oblique = Ray::new(Point::origin(), Vector3::new(12.0/5.0 + delta, 0.0, 9.0/5.0).normalize());
        assert!(scene.intersect(ray_near_oblique).is_none());
        let ray_hit_oblique = Ray::new(Point::origin(), Vector3::new(12.0/5.0, 0.0, 9.0/5.0).normalize());
        assert!(scene.intersect(ray_hit_oblique).is_some());
    }

    #[test]
    fn test_scene_intersect_depth() {
        let p1 = Object {shape: box Plane::from_abcd(0.0f32, 0.0, -1.0, 1.0), material: box TestMaterial};
        let p2 = Object {shape: box Plane::from_abcd(0.0f32, 0.0, -1.0, 2.0), material: box TestMaterial};
        let scene = Scene {objects: vec![p1, p2], light_sources: vec![]};
        let ray = Ray::new(Point::origin(), Vector3::new(0.0, 0.0, 1.0));
        assert!(Point3::new(0.0, 0.0, 1.0) == get_point(&scene, ray));
    }
}