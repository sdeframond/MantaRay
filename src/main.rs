#![feature(macro_rules)]

extern crate cgmath;
extern crate image;
extern crate num;

use cgmath::point::Point3;
use std::io::File;

use camera::OriginCamera;
use light::{Light, LightSource};
use material::{EmitterMaterial, DiffuseMaterial};
use object::Object;
use scene::Scene;
use shape::{Sphere, Plane};

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
    let sphere = Object {
        shape: box Sphere {center: Point3::new(0.0f32, 0.0, 5.0), radius: 1.0},
        material: box DiffuseMaterial { diffuse: Light::white(0.6), specular: Light::white(0.4), shininess: 50.0 }
    };
    let plane = Object {
        shape: box Plane::from_abcd(0.0f32, -1.0, 0.0, 0.8),
        material: box DiffuseMaterial { diffuse: Light::white(0.6), specular: Light::white(0.4), shininess: 50.0 }
    };
    let (light_src1, l1) = make_light_source(-2.0, -2.0, 4.0, 4.0, 2.0, 2.0);
    let (light_src2, l2) = make_light_source(2.0, -1.0, 5.0, 2.0, 4.0, 2.0);
    Scene {
        objects: vec![sphere, plane, l1, l2],
        light_sources: vec![box light_src1, box light_src2]
    }
}

fn make_light_source(x: f32, y: f32, z: f32, red: f32, green: f32, blue: f32) -> (LightSource, Object) {
    let position = Point3::new(x, y, z);
    let power = Light::new(red, green, blue);
    let light_mat = EmitterMaterial::new(red/5.0, green/5.0, blue/5.0);
    let obj = Object {
        shape: box Sphere {center: position, radius: 0.1},
        material: box light_mat
    };
    let ls = LightSource::new(position, power);
    (ls, obj)
}