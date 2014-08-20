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

type IntersectionInfo<'r> = (&'r Object, Point3<f32>);

impl Scene {
    pub fn background(&self, _direction: Vector3<f32>) -> Light {
        Light::zero()
    }

    pub fn intersect(&self, ray: Ray3<f32>) -> Option<IntersectionInfo> {
        let mut intersections: Vec<IntersectionInfo> = self.objects.iter()
            .map(|obj| obj.intersect(ray))
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();
        let distance_cmp = |v1: &IntersectionInfo, v2: &IntersectionInfo| {
            cmp_float(
                ray.origin.sub_p(v2.ref1()).length(),
                ray.origin.sub_p(v1.ref1()).length()
            )
        };
        intersections.sort_by(distance_cmp);
        intersections.pop()
    }

    pub fn shadow_intersect(&self, ray: Ray3<f32>, length: f32) -> bool {
        self.objects.iter().any(|obj| {
            obj.shadow_intersect(ray, length)
        })
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
    fn test_scene_intersect() {
        let p1 = Object {shape: box Plane::from_abcd(0.0f32, 0.0, -1.0, 1.0), material: box TestMaterial};
        let p2 = Object {shape: box Plane::from_abcd(0.0f32, 0.0, -1.0, 2.0), material: box TestMaterial};
        let scene = Scene {objects: vec![p1, p2], light_sources: vec![]};
        let ray = Ray::new(Point::origin(), Vector3::new(0.0, 0.0, 1.0));
        assert!(Point3::new(0.0, 0.0, 1.0) == get_point(&scene, ray));
    }
}