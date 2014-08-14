use light::Light;
use cgmath::vector::{dot, Vector, Vector3};

pub trait Material {
    fn emittance(&self) -> Light;
    fn reflectance(&self, normal: Vector3<f32>, dir_in: Vector3<f32>, dir_out: Vector3<f32>) -> Light;
}

pub struct DiffuseMaterial {
    pub diffuse: Light,
    pub specular: Light,
    pub shininess: f32
}

impl Material for DiffuseMaterial {
    fn emittance(&self) -> Light {
        Light::new(0.0, 0.0, 0.0)
    }

    fn reflectance(&self, n: Vector3<f32>, dir_in: Vector3<f32>, dir_out: Vector3<f32>) -> Light {
        let proj = dot(n, -dir_in);
        if proj > 0.0 {
            let diffuse = self.diffuse.mul_s(proj);
            let dir_in_reflection = dir_in.add_v(&n.mul_s(2.0).mul_s(proj));
            let alignment = dot(dir_out, dir_in_reflection);
            let specular = self.specular.mul_s(alignment.powf(self.shininess));
            diffuse.add(specular)
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
        let mat = DiffuseMaterial {
            diffuse: Light::new(1.0, 1.0, 1.0),
            specular: Light::new(0.0, 0.0, 0.0),
            shininess: 0.0
        };
        let normal = Vector3::new(1.0, 0.0, 0.0);
        let dir_in = Vector3::new(1.0, 0.0, 0.0);
        let dir_out = Vector3::new(1.0, 0.0, 0.0);
        let res = mat.reflectance(normal, dir_in, dir_out);
        assert!(res == Light::zero());
        let res = mat.reflectance(normal, -dir_in, dir_out);
        assert!(res != Light::zero());
    }
}