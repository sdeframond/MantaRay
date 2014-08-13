extern crate image;
extern crate cgmath;
extern crate num;

use std::cmp::max;
use cgmath::point::{Point, Point3};
use cgmath::ray::{Ray, Ray3};
use cgmath::intersect::Intersect;
use cgmath::sphere::Sphere;
use cgmath::vector::{EuclideanVector, Vector, Vector3};
use image::GenericImage;
use std::cmp::PartialOrd;
use std::io::File;
use std::num::Bounded;

struct Scene {
    objects: Vec<Object>,
    light_sources: Vec<Box<LightSource>>
}

impl Scene {
    fn background(&self, _direction: Vector3<f32>) -> Light {
        Light::new(0.0, 0.0, 0.0)
    }

    fn intersect(&self, ray: Ray3<f32>) -> Option<(&Object, Point3<f32>)> {
        let mut intersections: Vec<(&Object, Point3<f32>)> = self.objects.iter()
            .map(|obj| obj.intersect(ray))
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();
        let distance_cmp = |v1: &(&Object, Point3<f32>), v2: &(&Object, Point3<f32>)| {
            cmp_float(
                ray.origin.sub_p(v1.ref1()).length(),
                ray.origin.sub_p(v2.ref1()).length()
            )
        };
        intersections.sort_by(distance_cmp);
        intersections.pop()
    }
}

fn cmp_float<F: PartialOrd>(f1: F, f2: F) -> Ordering {
    match f1.partial_cmp(&f2) {
        None => Less,
        Some(ord) => ord
    }
}

struct Object {
    shape: Box<Shape>,
    material: Material
}

impl Object {
    fn emittance(&self, p: Point3<f32>) -> Light {
        self.material.emittance(p)
    }

    fn intersect(&self, ray: Ray3<f32>) -> Option<(&Object, Point3<f32>)> {
        match self.shape.intersect(ray) {
            None => None,
            Some(point) => Some((self, point))
        }
    }
}

struct LightSource {
    origin: Point3<f32>,
    light: Light
}

struct Light {
    red: f32,
    green: f32,
    blue: f32
}

impl Light {
    fn new(red: f32, green: f32, blue: f32) -> Light {
        Light {red: red, green: green, blue: blue}
    }

    fn add(&self, other: Light) -> Light {
        Light::new(self.red + other.red, self.green + other.green, self.blue + other.blue)
    }
}

trait Shape {
    fn intersect(&self, Ray3<f32>) -> Option<Point3<f32>>;
}

impl Shape for Sphere<f32> {
    fn intersect(&self, ray: Ray3<f32>) -> Option<Point3<f32>> {
        (*self, ray).intersection()
    }
    
}

struct Material;

impl Material {
    fn emittance(&self, _p: Point3<f32>) -> Light {
        Light::new(0.0, 0.0, 0.0)
    }
}

trait RayMaker {
    fn make_ray(&self, x: u32, y: u32) -> Ray3<f32>;
}

struct OriginCamera {
    aperture: f32,
    height: u32,
    width: u32
}

impl RayMaker for OriginCamera {
    fn make_ray(&self, x: u32, y: u32) -> Ray3<f32> {
        let maximum = max(x, y) as f32;
        let xx = self.aperture * ((x as f32) - (self.width as f32)/2.0)/maximum ;
        let yy = self.aperture * ((y as f32) - (self.height as f32)/2.0)/maximum;
        let v = Vector3::new(xx, yy, 1.0).normalize();
        Ray::new(Point3::new(0.0, 0.0, 0.0), v)
    }
}

#[test]
fn test_origin_camera_make_ray() {
    let cam = OriginCamera {aperture: 1.0, height: 1000, width: 1000};
    let ray_center = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    let ray_corner = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.5, 0.5, 1.0).normalize());
    assert!(ray_center == cam.make_ray(500, 500));
    assert!(ray_corner == cam.make_ray(1000, 1000));
    let cam2 = OriginCamera {aperture: 2.0, height: 1000, width: 1000};
    let ray_corner2 = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0).normalize());
    assert!(ray_corner2 == cam2.make_ray(1000, 1000));
}

fn main() {
    let scene = make_test_scene();
    let camera = make_test_camera();
    let (width, height) = (1000, 1000);
    let mut imbuf = image::ImageBuf::new(width, height);
    let pixel_renderer = |x, y| render_pixel(&camera, &scene, x, y);
    render_image(&mut imbuf, pixel_renderer);
    let fout = File::create(&Path::new("result.png")).unwrap();
    let _ = image::ImageRgb8(imbuf).save(fout, image::PNG);
}

fn make_test_camera() -> OriginCamera {
    OriginCamera {aperture: 2.0, height: 1000, width: 1000}
}

fn make_test_scene() -> Scene {
    let obj = Object {
        shape: box Sphere {center: Point3::new(0.0f32, 0.0, 5.0), radius: 3.0},
        material: Material
    };
    Scene {
        objects: vec![obj],
        light_sources: vec![]
    }
}

fn render_pixel<T: RayMaker>(ray_maker: &T, scene: &Scene, x: u32, y: u32) -> image::Rgb<u8> {
    let ray = ray_maker.make_ray(x, y);
    color_from_light(trace_path(scene, ray))
}

fn color_from_light(light: Light) -> image::Rgb<u8> {
    image::Rgb(convert(light.red), convert(light.green), convert(light.blue))
}

#[test]
fn color_from_light_test() {
    let light = Light::new(0.5, 0.5, 0.5);
    let color = image::Rgb(128u8, 128, 128);
    assert!(color_from_light(light) == color);
}

fn convert(x: f32) -> u8 {
    let low: u8 = Bounded::min_value();
    let high: u8 = Bounded::max_value();
    let scaled = x * high as f32;
    let fenced = scaled.max(low as f32).min(high as f32);
    fenced.round() as u8
}

fn trace_path(scene: &Scene, ray: Ray3<f32>) -> Light {
    match scene.intersect(ray) {
        None => scene.background(ray.direction),
        Some((object, point)) => {
            let reflected_light = Light::new(0.5, 0.5, 0.5);
            object.emittance(point).add(reflected_light)
        }
    }
}

type PixelRenderer<'a> = |u32, u32|:'a -> image::Rgb<u8>;

fn render_image(buffer: &mut image::ImageBuf<image::Rgb<u8>>, render_pixel: PixelRenderer) {
    let (width, height) = buffer.dimensions();
    for y in range(0, height) {
        for x in range(0, width) {
            let pixel = render_pixel(x, y);
            buffer.put_pixel(x, y, pixel);
        }
    }
}

#[test]
fn test_render_image() {
    let mut count = 0i;
    let mut imbuf: image::ImageBuf<image::Rgb<u8>> = image::ImageBuf::new(100, 100);
    { // We need a scope here because we are borrowing `count`.
        let renderer = |_, _| {count += 1; image::Rgb(0u8, 0, 0)};
        render_image(&mut imbuf, renderer);
    } // Now we can use `count`.
    assert!(count == 100 * 100);
    assert!(imbuf.get_pixel(34, 21) == image::Rgb(0, 0, 0));
}