extern crate image;
extern crate cgmath;
extern crate num;

use std::cmp::max;
use cgmath::point::{Point, Point3};
use cgmath::ray::{Ray, Ray3};
use cgmath::intersect::Intersect;
use cgmath::sphere::Sphere;
use cgmath::vector::{EuclideanVector, Vector3};
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

fn main() {
    let scene = make_scene();
    let imbuf = render_image(&scene, 800, 800);
    let fout = File::create(&Path::new("result.png")).unwrap();
    let _ = image::ImageRgb8(imbuf).save(fout, image::PNG);
}

fn make_scene() -> Scene {
    let obj = Object {
        shape: box Sphere {center: Point3::new(0.0f32, 0.0, 10.0), radius: 4.0},
        material: Material
    };
    Scene {
        objects: vec![obj],
        light_sources: vec![]
    }
}

fn render_image(scene: &Scene, width: u32, height: u32) -> image::ImageBuf<image::Rgb<u8>> {
    let mut imbuf = image::ImageBuf::new(width, height);
    let camera = OriginCamera{aperture: 1.0, width: width, height: height};
    for y in range(0, height) {
        for x in range(0, width) {
            let pixel = color_from_light(render_pixel(camera, scene, x, y));
            imbuf.put_pixel(x, y, pixel);
        }
    }
    imbuf
}

fn color_from_light(light: Light) -> image::Rgb<u8> {
    image::Rgb(convert(light.red), convert(light.green), convert(light.blue))
}

fn convert(x: f32) -> u8 {
    let low: u8 = Bounded::min_value();
    let high: u8 = Bounded::max_value();
    let scaled = x * high as f32;
    let fenced = scaled.max(low as f32).min(high as f32);
    fenced.round() as u8
}

fn render_pixel<T: RayMaker>(ray_maker: T, scene: &Scene, x: u32, y: u32) -> Light {
    let ray = ray_maker.make_ray(x, y);
    trace_path(scene, ray)
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