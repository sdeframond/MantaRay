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

    pub fn intersect(&self, ray: Ray3<f32>) -> Option<(&Object, &Shape, Point3<f32>)> {
        match self.shape.intersect(ray) {
            None => None,
            Some((shape, point)) => Some((self, shape, point))
        }
    }

    pub fn shadow_intersect(&self, shape: &Shape, ray: Ray3<f32>, length: f32) -> bool {
        self.shape.shadow_intersect(shape, ray, length)
    }

    pub fn normal(&self, point: Point3<f32>) -> Vector3<f32> {
        self.shape.normal(point)
    }
}