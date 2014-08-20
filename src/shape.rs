use cgmath::point::{Point, Point3};
use cgmath::ray::{Ray, Ray3};
use cgmath::intersect::Intersect;
use cgmath::vector::{EuclideanVector, Vector, Vector3};
use std::num::zero;

pub use cgmath::sphere::Sphere;
pub use cgmath::plane::Plane;

pub trait Shape {
    fn intersect(&self, Ray3<f32>) -> Option<Point3<f32>>;
    fn normal(&self, Point3<f32>) -> Vector3<f32>;
    fn shadow_intersect(&self, ray: Ray3<f32>, length: f32) -> bool {
        match self.intersect(ray) {
            None => false,
            Some(p) => p.sub_p(&ray.origin).length() < length
        }
    }
}

macro_rules! cgmath_intersect(
    () => (
        fn intersect(&self, ray: Ray3<f32>) -> Option<Point3<f32>> {
            let moved_ray = Ray::new(ray.origin.add_v(&ray.direction.mul_s(0.0001)), ray.direction);
            (*self, moved_ray).intersection()
        }
    )
)

impl Shape for Sphere<f32> {

    fn intersect(&self, ray: Ray3<f32>) -> Option<Point3<f32>> {
        let moved_ray = Ray::new(ray.origin.add_v(&ray.direction.mul_s(0.0001)), ray.direction);
        let l = self.center.sub_p(&moved_ray.origin);
        let tca = l.dot(&moved_ray.direction);
        if tca < zero() && l.length2() > self.radius*self.radius { return None; }
        let d2 = l.dot(&l) - tca*tca;
        if d2 > self.radius*self.radius { return None; }
        let thc = (self.radius*self.radius - d2).sqrt();
        let k = if tca > thc { tca - thc } else { tca + thc };
        Some(moved_ray.origin.add_v(&moved_ray.direction.mul_s(k)))
    }

    fn normal(&self, point: Point3<f32>) -> Vector3<f32> {
        point.sub_p(&self.center).normalize()
    }
}

impl Shape for Plane<f32> {

    cgmath_intersect!()

    fn normal(&self, _: Point3<f32>) -> Vector3<f32> {
        self.n.normalize()
    }
}

#[test]
fn test_sphere_normal() {
    let sphere = Sphere {center: Point::origin(), radius: 1.0};
    let n = sphere.normal(Point3::new(1.0, 0.0, 0.0));
    assert!(n == Vector3::new(1.0, 0.0, 0.0));
}

#[test]
fn test_sphere_intersect_from_outside() {
    let sphere = Sphere {center: Point::origin(), radius: 1.0};
    let p = Point3::new(-2.0, 0.0, 0.0);
    let dir = Vector3::new(1.0, 0.0, 0.0);
    let ray = Ray::new(p, dir);
    println!("sphere.intersect(ray) = {}", sphere.intersect(ray));
    assert!(sphere.intersect(ray) == Some(Point3::new(-1.0, 0.0, 0.0)));
}

#[test]
fn test_sphere_intersect_from_inside() {
    let sphere = Sphere {center: Point::origin(), radius: 1.0};
    let p = Point::origin();
    let dir = Vector3::new(1.0, 0.0, 0.0);
    let ray = Ray::new(p, dir);
    println!("sphere.intersect(ray) = {}", sphere.intersect(ray));
    assert!(sphere.intersect(ray) == Some(Point3::new(1.0, 0.0, 0.0)));
}

#[test]
fn test_plane_normal_length_is_one() {
    let p = Plane::from_abcd(1.0f32, 1.0, 1.0, 0.0);
    let n = p.normal(Point3::new(0.0, 0.0, 0.0));
    let delta = 0.0000001;
    assert!(n.length() < 1.0 + delta);
    assert!(n.length() > 1.0 - delta);
}