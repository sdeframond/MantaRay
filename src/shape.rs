use cgmath::point::{Point, Point3};
use cgmath::ray::Ray3;
use cgmath::intersect::Intersect;
use cgmath::vector::{EuclideanVector, Vector3};

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
            match (*self, ray).intersection() {
                None => None,
                Some(point) => if point.sub_p(&ray.origin).length() > 0.001 {
                    Some(point)
                } else {
                    None
                }
            }
        }
    )
)

impl Shape for Sphere<f32> {

    cgmath_intersect!()

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
fn test_plane_normal_length_is_one() {
    let p = Plane::from_abcd(1.0f32, 1.0, 1.0, 0.0);
    let n = p.normal(Point3::new(0.0, 0.0, 0.0));
    let delta = 0.0000001;
    assert!(n.length() < 1.0 + delta);
    assert!(n.length() > 1.0 - delta);
}