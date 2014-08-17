#![feature(macro_rules)]

extern crate cgmath;
extern crate image;
extern crate num;

use cgmath::point::Point3;
use std::io::File;
// use image::GenericImage;
// use std::sync::{Arc, Future, RWLock, TaskPool};
// use std::iter::count;
// use std::cmp::min;

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

// fn main() {
//     let scene = Arc::new(make_scene());
//     let (width, height) = (1000, 1000);
//     let camera = OriginCamera {aperture: 1.0, height: width, width: height};
//     let mut futures = Vec::from_fn((width*height) as uint, |i| {
//         let task_scene = scene.clone();
//         Future::spawn(proc() {
//             let x = (i as u32) % width;
//             let y = (i as u32) / width;
//             render::pixel(&camera, task_scene.deref(), x, y)
//         })
//     });

//     let pixel_renderer = |x: u32, y: u32| futures.get_mut((x + y*width) as uint).get();
//     let imbuf = render::image(width, height, pixel_renderer);
//     let fout = File::create(&Path::new("result.png")).unwrap();
//     let _ = image::ImageRgb8(imbuf).save(fout, image::PNG);
// }

// fn main() {
//     let scene = Arc::new(make_scene());
//     let (width, height) = (1000, 1000);
//     let camera = OriginCamera {aperture: 1.0, height: width, width: height};
//     let futbuf_max_size = 1000u32;
//     let mut imbuf = image::ImageBuf::new(width, height);
//     let mut allocated_futures = 0u32;
//     while allocated_futures < width*height {
//         let futbuf_size = min(futbuf_max_size, width*height - allocated_futures);
//         let mut futures = Vec::from_fn(futbuf_size as uint, |i| {
//             let task_scene = scene.clone();
//             Future::spawn(proc() {
//                 let x = (i as u32) % width;
//                 let y = (i as u32) / width;
//                 render::pixel(&camera, task_scene.deref(), x, y)
//             })
//         });
//         allocated_futures += futbuf_size;
//         for (i, future) in count(0u32, 1).zip(futures.mut_iter()) {
//             let x = i % width;
//             let y = i / width;
//             imbuf.put_pixel(x, y, future.get());
//         }
//     }
//     println!("allocated: {}", allocated_futures);

//     let fout = File::create(&Path::new("result.png")).unwrap();
//     let _ = image::ImageRgb8(imbuf).save(fout, image::PNG);
// }

// fn main() {
//     let (width, height) = (1000, 1000);
//     let camera = OriginCamera {aperture: 1.0, height: width, width: height};
//     let scene = Arc::new(make_scene());
//     let imlock = Arc::new(RWLock::new(image::ImageBuf::new(width, height)));
//     let mut pool = TaskPool::new(1, || proc(_tid) { () } );
//     for y in range(0, height) {
//         for x in range(0, width) {
//             let task_imlock = imlock.clone();
//             let task_scene = scene.clone();
//             pool.execute(proc(_) {
//                 let pixel = render::pixel(&camera, task_scene.deref(), x, y);
//                 let mut imbuf = task_imlock.write();
//                 imbuf.put_pixel(x, y, pixel);
//                 imbuf.downgrade();
//             });
//         }
//     }

//     let fout = File::create(&Path::new("result.png")).unwrap();
//     let _ = image::ImageRgb8(imlock.read().clone()).save(fout, image::PNG);
// }

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
        objects: vec![sphere, plane],
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