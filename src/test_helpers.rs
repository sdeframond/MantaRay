use cgmath::sphere::Sphere;
use cgmath::point::Point3;

use scene::Scene;
use object::Object;
use material::TestMaterial;
use camera::OriginCamera;

pub fn make_test_scene() -> Scene {
    let obj = Object {
        shape: box Sphere {center: Point3::new(0.0f32, 0.0, 5.0), radius: 3.0},
        material: box TestMaterial
    };
    Scene {
        objects: vec![obj],
        light_sources: vec![]
    }
}

pub fn make_test_camera() -> OriginCamera {
    OriginCamera {aperture: 2.0, height: 1000, width: 1000}
}