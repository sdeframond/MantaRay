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

#[test]
fn test_scene_intersect() {
    let delta = 0.000001;
    let get_point = |scene: &Scene, ray| { scene.intersect(ray).unwrap().val1() };
    let sphere = box Sphere {center: Point3::new(0.0f32, 0.0, 5.0), radius: 4.0};
    let obj = Object {shape: sphere, material: Material};
    let scene = Scene {objects: vec![obj], light_sources: vec![]};
    let ray_hit = Ray::new(Point::origin(), Vector3::new(0.0, 0.0, 1.0));
    assert!(Point3::new(0.0, 0.0, 1.0) == get_point(&scene, ray_hit));
    let ray_miss = Ray::new(Point3::new(10.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    assert!(scene.intersect(ray_miss).is_none());
    let ray_border1 = Ray::new(Point3::new(4.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    assert!(Point3::new(4.0, 0.0, 5.0) == get_point(&scene, ray_border1));
    let ray_near1 = Ray::new(Point3::new(4.0 + delta, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    assert!(scene.intersect(ray_near1).is_none());
    let ray_near_oblique = Ray::new(Point::origin(), Vector3::new(12.0/5.0 + delta, 0.0, 9.0/5.0).normalize());
    assert!(scene.intersect(ray_near_oblique).is_none());
    let ray_hit_oblique = Ray::new(Point::origin(), Vector3::new(12.0/5.0, 0.0, 9.0/5.0).normalize());
    assert!(scene.intersect(ray_hit_oblique).is_some());
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

#[deriving(PartialEq)]
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
        let maximum = max(self.width, self.height) as f32;
        let to_dim = |val: f32, range: f32| self.aperture * (val - range/2.0) / maximum;
        let xx = to_dim(x as f32, self.width as f32);
        let yy = to_dim(y as f32, self.height as f32);
        let v = Vector3::new(xx, yy, 1.0).normalize();
        Ray::new(Point::origin(), v)
    }
}

#[test]
fn test_origin_camera_make_ray() {
    let cam = OriginCamera {aperture: 1.0, height: 1000, width: 1000};
    let ray_from_origin = |x, y, z| Ray::new(Point::origin(), Vector3::new(x, y, z).normalize());
    let ray_center = ray_from_origin(0.0, 0.0, 1.0);
    let ray_corner = ray_from_origin(0.5, 0.5, 1.0);
    let ray_corner1 = ray_from_origin(-0.5, -0.5, 1.0);
    assert!(ray_center == cam.make_ray(500, 500));
    assert!(ray_corner == cam.make_ray(1000, 1000));
    assert!(ray_corner1 == cam.make_ray(0, 0));
    let cam2 = OriginCamera {aperture: 2.0, height: 1000, width: 1000};
    let ray_corner2 = ray_from_origin(1.0, 1.0, 1.0);
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

#[test]
fn test_render_pixel() {
    let scene = make_test_scene();
    let camera = make_test_camera();
    let black = image::Rgb(0, 0, 0);
    assert!(black == render_pixel(&camera, &scene, 1, 1));
    assert!(black == render_pixel(&camera, &scene, 1000, 1));
    assert!(black == render_pixel(&camera, &scene, 1, 1000));
    assert!(black == render_pixel(&camera, &scene, 1000, 1000));
    assert!(black != render_pixel(&camera, &scene, 500, 500));
    assert!(black == render_pixel(&camera, &scene, 500, 124));
    assert!(black != render_pixel(&camera, &scene, 500, 125));
    assert!(black == render_pixel(&camera, &scene, 500, 1000-124));
    assert!(black != render_pixel(&camera, &scene, 500, 1000-125));
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
            let reflected_light = Light::new(1.0, 1.0, 1.0);
            object.emittance(point).add(reflected_light)
        }
    }
}

#[test]
fn test_trace_path() {
    let delta = 0.000001f32;
    let assert_tp = |scene: &Scene, ray, r, g, b| assert!(Light::new(r, g, b) == trace_path(scene, ray));
    let scene = make_test_scene();
    let origin = Point::origin();
    let ray_miss = Ray::new(origin, Vector3::new(12.0/5.0 + delta, 0.0, 16.0/5.0).normalize());
    assert_tp(&scene, ray_miss, 0.0, 0.0, 0.0);
    let ray_hit = Ray::new(origin, Vector3::new(12.0/5.0, 0.0, 16.0/5.0).normalize());
    assert_tp(&scene, ray_hit, 1.0, 1.0, 1.0);
    let ray_miss2 = Ray::new(origin, Vector3::new(-12.0/5.0 - delta, 0.0, 16.0/5.0).normalize());
    assert_tp(&scene, ray_miss2, 0.0, 0.0, 0.0);
    let ray_hit2 = Ray::new(origin, Vector3::new(-12.0/5.0, 0.0, 16.0/5.0).normalize());
    assert_tp(&scene, ray_hit2, 1.0, 1.0, 1.0);
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