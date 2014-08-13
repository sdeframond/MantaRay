#![feature(globs)]
#![feature(macro_rules)]

extern crate cgmath;
extern crate image;
extern crate num;

use cgmath::point::Point3;
use cgmath::sphere::Sphere;
use std::io::File;

use camera::*;
use light::*;
use material::*;
use object::*;
use render::*;
use scene::*;

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
    let scene = make_diffuse_scene();
    let (width, height) = (1000, 1000);
    let camera = OriginCamera {aperture: 2.0, height: width, width: height};
    let pixel_renderer = |x, y| render_pixel(&camera, &scene, x, y);
    let imbuf = render_image(width, height, pixel_renderer);
    let fout = File::create(&Path::new("result.png")).unwrap();
    let _ = image::ImageRgb8(imbuf).save(fout, image::PNG);
}

fn make_diffuse_scene() -> Scene {
    let obj = Object {
        shape: box Sphere {center: Point3::new(0.0f32, 0.0, 5.0), radius: 3.0},
        material: box DiffuseMaterial { diffuse: Light::new(1.0, 1.0, 1.0) }
    };
    Scene {
        objects: vec![obj],
        light_sources: vec![]
    }
}