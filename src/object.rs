use cgmath::vector::Vector3;
use cgmath::ray::Ray3;
use cgmath::point::Point3;

use shape::*;
use material::*;
use light::*;

pub struct Object {
    pub shape: Box<Shape>,
    pub material: Box<Material>
}

impl Object {
    pub fn emittance(&self, _: Point3<f32>) -> Light {
        self.material.emittance()
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

    pub fn normal(&self, point: Point3<f32>) -> Vector3<f32> {
        self.shape.normal(point)
    }
}