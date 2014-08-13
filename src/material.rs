use light::Light;
use cgmath::vector::{Vector, Vector3};

pub trait Material {
    fn emittance(&self) -> Light;
    fn reflectance(&self, normal: Vector3<f32>, dir_in: Vector3<f32>, dir_out: Vector3<f32>) -> Light;
}

pub struct DiffuseMaterial {
    pub diffuse: Light
}

impl Material for DiffuseMaterial {
    fn emittance(&self) -> Light {
        Light::new(0.0, 0.0, 0.0)
    }

    fn reflectance(&self, n: Vector3<f32>, dir_in: Vector3<f32>, dir_out: Vector3<f32>) -> Light {
        self.diffuse.mul_s(n.dot(&dir_out))
    }
}

pub struct TestMaterial;

impl Material for TestMaterial {
    fn emittance(&self) -> Light {
        Light::new(0.0, 0.0, 0.0)
    }

    fn reflectance(&self, _: Vector3<f32>, _: Vector3<f32>, _: Vector3<f32>) -> Light {
        Light::new(1.0, 1.0, 1.0)
    }
}