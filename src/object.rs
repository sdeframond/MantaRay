use cgmath::vector::Vector3;
use cgmath::ray::Ray3;
use cgmath::point::Point3;

use shape::Shape;
use material::Material;
use light::Light;

pub struct Object {
    pub shape: Box<Shape>,
    pub material: Box<Material>
}

impl Object {
    pub fn emittance(&self, p: Point3<f32>, dir: Vector3<f32>) -> Light {
        self.material.emittance(self.normal(p), dir)
    }
    pub fn reflectance(&self, p: Point3<f32>, dir_in: Vector3<f32>, dir_out: Vector3<f32>) -> Light {
        self.material.reflectance(self.normal(p), dir_in, dir_out)
    }
    pub fn intersect(&self, ray: Ray3<f32>) -> Option<(&Object, Point3<f32>)> {
        match self.shape.intersect(ray) {
            None => None,
            Some(point) => Some((self, point))
        }
    }
    pub fn shadow_intersect(&self, ray: Ray3<f32>, length: f32) -> bool {
        self.shape.shadow_intersect(ray, length)
    }
    pub fn normal(&self, point: Point3<f32>) -> Vector3<f32> {
        self.shape.normal(point)
    }
    pub fn next_step(&self, point: Point3<f32>, dir_in: Vector3<f32>, tracer: |Ray3<f32>| -> Light) -> Light {
        self.material.next_step(point, self.normal(point), dir_in, tracer)
    }
}