use cgmath::point::{Point, Point3};
use cgmath::ray::Ray3;
use cgmath::intersect::Intersect;
use cgmath::vector::{EuclideanVector, Vector3};
use std::any::{Any, AnyRefExt};

pub use cgmath::sphere::Sphere;
pub use cgmath::plane::Plane;

pub trait Shape : PrivateShape {
    fn intersect(&self, Ray3<f32>) -> Option<(&Shape, Point3<f32>)>;
    fn normal(&self, Point3<f32>) -> Vector3<f32>;
    fn intersect_except_shape(&self, shape: &Shape, ray: Ray3<f32>) -> bool {
        !self.equals_shape(shape) && self.intersect(ray).is_some()
    }
}

trait PrivateShape {
    fn as_any(&self) -> &Any;
    fn equals_shape(&self, &Shape) -> bool;
}

impl Shape for Sphere<f32> {
    fn intersect(&self, ray: Ray3<f32>) -> Option<(&Shape, Point3<f32>)> {
        match (*self, ray).intersection() {
            None => None,
            Some(point) => Some((self as &Shape, point))
        }
    }

    fn normal(&self, point: Point3<f32>) -> Vector3<f32> {
        point.sub_p(&self.center).normalize()
    }
}

impl PrivateShape for Sphere<f32> {
    fn as_any(&self) -> &Any {
        self as &Any
    }

    fn equals_shape(&self, other: &Shape) -> bool {
        match other.as_any().downcast_ref::<Sphere<f32>>() {
            None => false,
            Some(sphere) => self == sphere
        }
    }
}

impl Shape for Plane<f32> {
    fn intersect(&self, ray: Ray3<f32>) -> Option<(&Shape, Point3<f32>)> {
        match (*self, ray).intersection() {
            None => None,
            Some(point) => Some((self as &Shape, point))
        }
    }

    fn normal(&self, _: Point3<f32>) -> Vector3<f32> {
        self.n
    }
}

impl PrivateShape for Plane<f32> {
    fn as_any(&self) -> &Any {
        self as &Any
    }

    fn equals_shape(&self, other: &Shape) -> bool {
        match other.as_any().downcast_ref::<Plane<f32>>() {
            None => false,
            Some(plane) => self == plane
        }
    }
}

#[test]
fn test_sphere_normal() {
    let sphere = Sphere {center: Point::origin(), radius: 1.0};
    let n = sphere.normal(Point3::new(1.0, 0.0, 0.0));
    assert!(n == Vector3::new(1.0, 0.0, 0.0));
}