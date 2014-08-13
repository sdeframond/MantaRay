use std::cmp::max;
use cgmath::point::{Point};
use cgmath::ray::{Ray, Ray3};
use cgmath::vector::{EuclideanVector, Vector3};

pub trait RayMaker {
    fn make_ray(&self, x: u32, y: u32) -> Ray3<f32>;
}

pub struct OriginCamera {
    pub aperture: f32,
    pub height: u32,
    pub width: u32
}

impl RayMaker for OriginCamera {
    fn make_ray(&self, x: u32, y: u32) -> Ray3<f32> {
        let maximum = max(self.width, self.height) as f32;
        let to_dim = |val: f32, range: f32| self.aperture * (val - range/2.0) / maximum;
        let xx = to_dim(x as f32, self.width as f32);
        let yy = to_dim(y as f32, self.height as f32);
        let v = Vector3::new(xx, yy, 1.0).normalize();
        Ray::new(Point::origin(), v)
    }
}

#[test]
fn test_origin_camera_make_ray() {
    let cam = OriginCamera {aperture: 1.0, height: 1000, width: 1000};
    let ray_from_origin = |x, y, z| Ray::new(Point::origin(), Vector3::new(x, y, z).normalize());
    let ray_center = ray_from_origin(0.0, 0.0, 1.0);
    let ray_corner = ray_from_origin(0.5, 0.5, 1.0);
    let ray_corner1 = ray_from_origin(-0.5, -0.5, 1.0);
    assert!(ray_center == cam.make_ray(500, 500));
    assert!(ray_corner == cam.make_ray(1000, 1000));
    assert!(ray_corner1 == cam.make_ray(0, 0));
    let cam2 = OriginCamera {aperture: 2.0, height: 1000, width: 1000};
    let ray_corner2 = ray_from_origin(1.0, 1.0, 1.0);
    assert!(ray_corner2 == cam2.make_ray(1000, 1000));
}