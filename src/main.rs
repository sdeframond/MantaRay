#![feature(macro_rules)]

extern crate cgmath;
extern crate image;
extern crate num;

use cgmath::point::Point3;
use cgmath::sphere::Sphere;
use std::io::File;

use camera::OriginCamera;
use light::{Light, LightSource};
use material::DiffuseMaterial;
use object::Object;
use scene::Scene;

mod camera;
mod light;
mod material;
mod object;
mod pathtracing;
mod render;
mod scene;
mod shape;
mod utils;
#[cfg(test)]
mod test_helpers;

fn main() {
    let scene = make_scene();
    let (width, height) = (1000, 1000);
    let camera = OriginCamera {aperture: 1.0, height: width, width: height};
    let pixel_renderer = |x, y| render::pixel(&camera, &scene, x, y);
    let imbuf = render::image(width, height, pixel_renderer);
    let fout = File::create(&Path::new("result.png")).unwrap();
    let _ = image::ImageRgb8(imbuf).save(fout, image::PNG);
}

fn make_scene() -> Scene {
    let obj = Object {
        shape: box Sphere {center: Point3::new(0.0f32, 0.0, 5.0), radius: 1.0},
        material: box DiffuseMaterial { diffuse: Light::new(1.0, 1.0, 1.0) }
    };
    let p1 = Point3::new(-2.0f32, -2.0, 4.0);
    let l1 = Object {
        shape: box Sphere {center: p1, radius: 0.1},
        material: box DiffuseMaterial { diffuse: Light::new(1.0, 1.0, 1.0) }
    };
    let light_src1 = box LightSource::new(p1, Light::new(2.0, 0.8, 0.8));
    let p2 = Point3::new(2.0f32, 1.0, 5.0);
    let l2 = Object {
        shape: box Sphere {center: p2, radius: 0.1},
        material: box DiffuseMaterial { diffuse: Light::new(1.0, 1.0, 1.0) }
    };
    let light_src2 = box LightSource::new(p2, Light::new(0.8, 2.0, 0.8));
    Scene {
        objects: vec![obj, l1, l2],
        light_sources: vec![light_src1, light_src2]
    }
}