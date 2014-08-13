use cgmath::point::{Point, Point3};
use cgmath::ray::Ray3;
use cgmath::intersect::Intersect;
use cgmath::sphere::Sphere;
use cgmath::vector::{EuclideanVector, Vector3};

pub trait Shape {
    fn intersect(&self, Ray3<f32>) -> Option<Point3<f32>>;
    fn normal(&self, Point3<f32>) -> Vector3<f32>;
}

impl Shape for Sphere<f32> {
    fn intersect(&self, ray: Ray3<f32>) -> Option<Point3<f32>> {
        (*self, ray).intersection()
    }
    fn normal(&self, point: Point3<f32>) -> Vector3<f32> {
        point.sub_p(&self.center).normalize()
    }
}

#[test]
fn test_sphere_normal() {
    let sphere = Sphere {center: Point::origin(), radius: 1.0};
    let n = sphere.normal(Point3::new(1.0, 0.0, 0.0));
    assert!(n == Vector3::new(1.0, 0.0, 0.0));
}