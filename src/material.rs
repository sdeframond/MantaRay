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

    fn reflectance(&self, n: Vector3<f32>, dir_in: Vector3<f32>, _dir_out: Vector3<f32>) -> Light {
        let dot = n.dot(&(-dir_in));
        if dot > 0.0 {
            self.diffuse.mul_s(dot)
        } else {
            Light::zero()
        }
    }
}

#[cfg(test)]
pub struct TestMaterial;

#[cfg(test)]
impl Material for TestMaterial {
    fn emittance(&self) -> Light {
        Light::new(1.0, 1.0, 1.0)
    }

    fn reflectance(&self, _: Vector3<f32>, _: Vector3<f32>, _: Vector3<f32>) -> Light {
        Light::new(1.0, 1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {

    use material::{Material, DiffuseMaterial};
    use cgmath::vector::Vector3;
    use light::Light;

    #[test]
    fn test_diffuse_material_reflectance() {
        let mat = DiffuseMaterial {diffuse: Light::new(1.0, 1.0, 1.0)};
        let normal = Vector3::new(1.0, 0.0, 0.0);
        let dir_in = Vector3::new(1.0, 0.0, 0.0);
        let dir_out = Vector3::new(1.0, 0.0, 0.0);
        let res = mat.reflectance(normal, dir_in, dir_out);
        assert!(res == Light::zero());
        let res = mat.reflectance(normal, -dir_in, dir_out);
        assert!(res != Light::zero());
    }
}