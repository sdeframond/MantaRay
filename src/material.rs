use light::Light;
use cgmath::vector::{dot, Vector, Vector3};

pub trait Material {
    fn emittance(&self, Vector3<f32>, Vector3<f32>) -> Light;
    fn reflectance(&self, normal: Vector3<f32>, dir_in: Vector3<f32>, dir_out: Vector3<f32>) -> Light;
}

pub struct DiffuseMaterial {
    pub diffuse: Light,
    pub specular: Light,
    pub shininess: f32
}

impl Material for DiffuseMaterial {
    fn emittance(&self, _n: Vector3<f32>, _dir: Vector3<f32>) -> Light {
        Light::new(0.0, 0.0, 0.0)
    }

    fn reflectance(&self, n: Vector3<f32>, dir_in: Vector3<f32>, dir_out: Vector3<f32>) -> Light {
        let proj = dot(n, -dir_in);
        if proj * dot(n, dir_out) > 0.0 {
            let proj = proj.abs();
            let diffuse = self.diffuse.mul_s(proj);
            let dir_in_reflection = dir_in.add_v(&n.mul_s(2.0).mul_s(proj));
            let alignment = dot(dir_out, dir_in_reflection);
            let specular = self.specular.mul_s(alignment.powf(self.shininess));
            diffuse + specular
        } else {
            Light::zero()
        }
    }
}

pub struct EmitterMaterial {
    emittance: Light
}

impl EmitterMaterial {
    pub fn new(r: f32, g: f32, b: f32) -> EmitterMaterial {
        EmitterMaterial { emittance: Light::new(r,g,b) }
    }
}

impl Material for EmitterMaterial {
    fn emittance(&self, n: Vector3<f32>, dir: Vector3<f32>) -> Light {
        self.emittance.mul_s(dot(n, dir))
    }

    fn reflectance(&self, _n: Vector3<f32>, _dir_in: Vector3<f32>, _dir_out: Vector3<f32>) -> Light {
        Light::zero()
    }
}

#[cfg(test)]
pub struct TestMaterial;

#[cfg(test)]
impl Material for TestMaterial {
    fn emittance(&self, _n: Vector3<f32>, _dir: Vector3<f32>) -> Light {
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
        assert!(res.red > 0.0);
        let res = mat.reflectance(normal, dir_in, -dir_out);
        assert!(res != Light::zero());
        assert!(res.red > 0.0);
        let res = mat.reflectance(normal, -dir_in, -dir_out);
        assert!(res == Light::zero());
    }
}