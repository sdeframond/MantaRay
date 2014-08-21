use light::Light;
use cgmath::vector::{dot, EuclideanVector, Vector, Vector3};
use cgmath::point::{Point, Point3};
use cgmath::ray::{Ray, Ray3};
use std::rand;
use std::rand::Rng;
use std::rand::distributions::{IndependentSample, Range};
use std::f32;

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
    // pub specular: Light,
    // pub shininess: f32
}

impl DiffuseMaterial {
    pub fn new(r: f32, g: f32, b: f32) -> DiffuseMaterial {
        DiffuseMaterial { diffuse: Light::new(r, g, b) }
    }
}

impl Material for DiffuseMaterial {
    fn reflectance(&self, n: Vector3<f32>, dir_in: Vector3<f32>, dir_out: Vector3<f32>) -> Light {
        let proj = dot(n, -dir_in);
        if proj * dot(n, dir_out) > 0.0 {
            let proj = proj.abs();
            self.diffuse.mul_s(proj)
            // let dir_in_reflection = dir_in.add_v(&n.mul_s(2.0).mul_s(proj));
            // let alignment = dot(dir_out, dir_in_reflection);
            // let specular = self.specular.mul_s(alignment.powf(self.shininess));
            // diffuse + specular
        } else {
            Light::zero()
        }
    }
}

pub struct GlobalDiffuseMaterial {
    diffuse: Light,
    n_rays: uint,
}

impl GlobalDiffuseMaterial {
    pub fn new(r: f32, g: f32, b:f32, n: uint) -> GlobalDiffuseMaterial {
        GlobalDiffuseMaterial { diffuse: Light::new(r,g,b), n_rays: n }
    }
}

fn unit_vec_from_angles(theta: f32, phi: f32) -> Vector3<f32> {
    Vector3::new(
        theta.sin() * phi.cos(),
        theta.sin() * phi.sin(),
        theta.cos()
    )
}

impl Material for GlobalDiffuseMaterial {
    fn reflectance(&self, n: Vector3<f32>, dir_in: Vector3<f32>, dir_out: Vector3<f32>) -> Light {
        let proj = dot(n, -dir_in);
        if proj * dot(n, dir_out) > 0.0 {
            let proj = proj.abs();
            self.diffuse.mul_s(proj)
        } else {
            Light::zero()
        }
    }
    fn next_step(&self, point: Point3<f32>, n: Vector3<f32>, dir_in: Vector3<f32>, tracer: |Ray3<f32>| -> Light) -> Light {
        let mut received = Light::zero();
        let proj_in = dot(n, dir_in);
        let mut rng = rand::task_rng();
        let between = Range::new(-f32::consts::PI, f32::consts::PI);
        for i in range(0, self.n_rays) {
            let mut dir_out = unit_vec_from_angles(between.ind_sample(&mut rng), between.ind_sample(&mut rng));
            let proj_out = dot(dir_out, n);
            if proj_out * proj_in > 0.0 {
                dir_out = -dir_out;
            }
            received = received + tracer(Ray::new(point, dir_out.normalize())).mul_s(proj_out.abs());
        }
        self.diffuse.mul_l(received.mul_s(1.0 / self.n_rays as f32))
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
    color: Light
}

impl ReflectiveMaterial {
    pub fn new(r: f32, g: f32, b: f32) -> ReflectiveMaterial {
        ReflectiveMaterial { color: Light::new(r,g,b) }
    }
}

impl Material for ReflectiveMaterial {
    fn next_step(&self, point: Point3<f32>, n: Vector3<f32>, dir_in: Vector3<f32>, tracer: |Ray3<f32>| -> Light) -> Light {
        let reflected_dir = dir_in.sub_v(&n.mul_s(2.0 * dot(n, dir_in)));
        let ray = Ray::new(point, reflected_dir);
        let reflected = tracer(ray);
        self.color.mul_l(reflected)
    }
}

pub struct RefractiveMaterial {
    color: Light,
    index: f32
}

impl RefractiveMaterial {
    pub fn new(r: f32, g: f32, b: f32, i: f32) -> RefractiveMaterial {
        RefractiveMaterial { color: Light::new(r,g,b), index: i }
    }
}

impl Material for RefractiveMaterial {
    fn next_step(&self, point: Point3<f32>, n: Vector3<f32>, dir_in: Vector3<f32>, tracer: |Ray3<f32>| -> Light) -> Light {
        let proj = dot(n, dir_in);
        let cos_theta1 = proj.abs();
        let r = if proj < 0.0 { 1.0/self.index } else { self.index };
        let squared_cos_theta2 = 1.0 - r.powi(2)*(1.0 - cos_theta1.powi(2));
        let refracted_dir = if squared_cos_theta2 > 0.0 {
            let cos_theta2 = squared_cos_theta2.sqrt();
            let reverse = if proj < 0.0 { 1.0 } else { -1.0 };
            let k = reverse*(r*cos_theta1 - cos_theta2);
            dir_in.mul_s(r).add_v(&n.mul_s(k))
        } else {
            // Just relfect the incoming ray.
            dir_in.sub_v(&n.mul_s(2.0 * proj))
        };
        let ray = Ray::new(point, refracted_dir);
        let reflected = tracer(ray);
        self.color.mul_l(reflected)
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

    use material::{Material, DiffuseMaterial, ReflectiveMaterial, RefractiveMaterial};
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
    fn test_refractive_material_next_step_outside_in() {
        let mat = RefractiveMaterial::new(1.0, 1.0, 0.0, 1.3);
        let p = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(1.0, 0.0, 0.0);
        let dir_in = -normal;
        let tracer = |ray: Ray3<f32>| {
            assert!(ray.origin == p);
            println!("ray.direction = {}", ray.direction);
            assert!(ray.direction == dir_in);
            Light::white(0.2)
        };
        let res = mat.next_step(p, normal, dir_in, tracer);
        assert!(res == Light::new(0.2, 0.2, 0.0))
    }

    #[test]
    fn test_refractive_material_next_step_outside_in_tangent() {
        let mat = RefractiveMaterial::new(1.0, 1.0, 0.0, 1.3);
        let p = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(1.0, 0.0, 0.0);
        let dir_in = Vector3::new(-0.00000000000001, 1.0, 0.0);
        let tracer = |ray: Ray3<f32>| {
            assert!(ray.origin == p);
            println!("ray.direction = {}", ray.direction);
            assert!(ray.direction.x < -0.6389);
            assert!(ray.direction.y > 0.7692);
            Light::white(0.2)
        };
        let res = mat.next_step(p, normal, dir_in, tracer);
        assert!(res == Light::new(0.2, 0.2, 0.0))
    }

    #[test]
    fn test_refractive_material_next_step_inside_out_tangent() {
        let mat = RefractiveMaterial::new(1.0, 1.0, 0.0, 1.3);
        let p = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(1.0, 0.0, 0.0);
        let dir_in = Vector3::new(0.00000000000001, 1.0, 0.0);
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
    fn test_refractive_material_next_step_inside_out() {
        let mat = RefractiveMaterial::new(1.0, 1.0, 0.0, 1.3);
        let p = Point3::new(0.0, 0.0, 0.0);
        let normal = Vector3::new(1.0, 0.0, 0.0);
        let dir_in = normal;
        let tracer = |ray: Ray3<f32>| {
            assert!(ray.origin == p);
            println!("ray.direction = {}", ray.direction);
            println!("dir_in = {}", dir_in);
            assert!(ray.direction == dir_in);
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