use light::Light;
use cgmath::vector::{dot, Vector, Vector3};
use cgmath::point::Point3;
use cgmath::ray::{Ray, Ray3};

pub trait Material {
    fn emittance(&self, _n: Vector3<f32>, _dir: Vector3<f32>) -> Light {
        Light::zero()
    }
    fn reflectance(&self, _normal: Vector3<f32>, _dir_in: Vector3<f32>, _dir_out: Vector3<f32>) -> Light {
        Light::zero()
    }
    fn next_step(&self, _point: Point3<f32>, _n: Vector3<f32>, _dir_in: Vector3<f32>, _tracer: |Ray3<f32>| -> Light) -> Light {
        Light::zero()
    }
}

pub struct DiffuseMaterial {
    pub diffuse: Light,
    pub specular: Light,
    pub shininess: f32
}

impl Material for DiffuseMaterial {
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
}

pub struct ReflectiveMaterial {
    reflection_color: Light
}

impl ReflectiveMaterial {
    pub fn new(r: f32, g: f32, b: f32) -> ReflectiveMaterial {
        ReflectiveMaterial { reflection_color: Light::new(r,g,b) }
    }
}

impl Material for ReflectiveMaterial {
    fn next_step(&self, point: Point3<f32>, n: Vector3<f32>, dir_in: Vector3<f32>, tracer: |Ray3<f32>| -> Light) -> Light {
        let reflected_dir = dir_in.sub_v(&n.mul_s(2.0 * dot(n, dir_in)));
        let ray = Ray::new(point, reflected_dir);
        let reflected = tracer(ray);
        self.reflection_color.mul_l(reflected)
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

    use material::{Material, DiffuseMaterial, ReflectiveMaterial};
    use cgmath::vector::Vector3;
    use cgmath::point::Point3;
    use cgmath::ray::Ray3;
    use light::Light;

    #[test]
    fn test_reflective_material_next_step() {
        let mat = ReflectiveMaterial::new(1.0, 1.0, 0.0);
        let p = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(1.0, 0.0, 0.0);
        let dir_in = Vector3::new(1.0, 1.0, 1.0);
        let tracer = |ray: Ray3<f32>| {
            assert!(ray.origin == p);
            assert!(ray.direction.x == -dir_in.x);
            assert!(ray.direction.y == dir_in.y);
            assert!(ray.direction.z == dir_in.z);
            Light::white(0.2)
        };
        let res = mat.next_step(p, normal, dir_in, tracer);
        assert!(res == Light::new(0.2, 0.2, 0.0))
    }

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