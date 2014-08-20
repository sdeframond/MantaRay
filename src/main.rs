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
use material::{EmitterMaterial, DiffuseMaterial, ReflectiveMaterial, RefractiveMaterial};
use object::Object;
use scene::Scene;
use shape::{Sphere, Plane};

mod camera;
mod light;
mod material;
mod object;
mod raytracing;
mod render;
mod scene;
mod shape;
mod utils;
#[cfg(test)]
mod test_helpers;

fn main() {
    let scene = make_scene();
    let (width, height) = (1000, 1000);
    let camera = OriginCamera {aperture: 1.5, height: width, width: height};
    let pixel_renderer = |x, y| render::pixel(&camera, &scene, x, y);
    let imbuf = render::image(width, height, pixel_renderer);
    let fout = File::create(&Path::new("result.png")).unwrap();
    let _ = image::ImageRgb8(imbuf).save(fout, image::PNG);
}

// // Currently runs 7x slower. No output.
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

// // Currently runs 6x slower.
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

// // Currently runs 1.3x to 10x slower, the more tasks in the pool the slower.
// fn main() {
//     let (width, height) = (1000, 1000);
//     let camera = OriginCamera {aperture: 1.0, height: width, width: height};
//     let scene = Arc::new(make_scene());
//     let imlock = Arc::new(RWLock::new(image::ImageBuf::new(width, height)));
//     let mut counter = Arc::new(RWLock::new(width * height));
//     let mut pool = TaskPool::new(100, || proc(_tid) { () } );
//     for y in range(0, height) {
//         for x in range(0, width) {
//             let task_imlock = imlock.clone();
//             let task_counter = counter.clone();
//             let task_scene = scene.clone();
//             pool.execute(proc(_) {
//                 let pixel = render::pixel(&camera, task_scene.deref(), x, y);
//                 {
//                     let mut imbuf = task_imlock.write();
//                     imbuf.put_pixel(x, y, pixel);
//                 }
//                 let mut count = task_counter.write();
//                 *count = *count - 1;
//             });
//         }
//     }
//     loop {
//         let c = *counter.read();
//         if c == 0 { break };
//         println!("c = {}, Waiting for 1s...", c);
//         std::io::timer::sleep( std::time::duration::Duration::seconds(1));
//     }
//     let fout = File::create(&Path::new("result.png")).unwrap();
//     let _ = image::ImageRgb8(imlock.read().clone()).save(fout, image::PNG);
// }

fn make_scene() -> Scene {
    let sphere = Object {
        shape: box Sphere {center: Point3::new(1.5f32, 2.5, 4.7), radius: 0.5},
        material: box DiffuseMaterial { diffuse: Light::new(0.0, 0.0, 0.6), specular: Light::white(0.4), shininess: 50.0 }
    };
    let mirror = Object {
        shape: box Sphere {center: Point3::new(-1.0f32, 1.5, 5.0), radius: 1.5},
        // material: box DiffuseMaterial { diffuse: Light::new(0.0, 0.0, 0.6), specular: Light::white(0.4), shininess: 50.0 }
        material: box ReflectiveMaterial::new(1.0, 0.9, 0.3)
    };
    let glass = Object {
        shape: box Sphere {center: Point3::new(0.8f32, 0.7, 3.7), radius: 0.7},
        // material: box DiffuseMaterial { diffuse: Light::new(0.0, 0.0, 0.6), specular: Light::white(0.4), shininess: 50.0 }
        material: box RefractiveMaterial::new(1.0, 1.0, 1.0, 1.4)
    };
    let bottom = make_plane(0.0f32, -1.0, 0.0, 3.0);
    let top = make_plane(0.0f32, 1.0, 0.0, 3.0);
    let right = make_plane(-1.0f32, 0.0, 0.0, 3.0);
    let left = make_plane(1.0f32, 0.0, 0.0, 3.0);
    let back = make_plane(0.0f32, 0.0, -1.0, 7.0);
    let (light_src1, l1) = make_light_source(-2.0, -2.0, 4.0, 2.0, 2.0, 2.0);
    let (light_src2, l2) = make_light_source(2.0, -1.0, 5.0, 2.0, 2.0, 2.0);
    Scene {
        objects: vec![bottom, top, left, right, back, sphere, mirror, glass],
        light_sources: vec![box light_src1, box light_src2]
    }
}

fn make_plane(a: f32, b: f32, c: f32, d: f32) -> Object {
    Object {
        shape: box Plane::from_abcd(a, b, c, d),
        material: box DiffuseMaterial { diffuse: Light::white(0.9), specular: Light::white(0.1), shininess: 50.0 }
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