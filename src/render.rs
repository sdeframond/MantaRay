extern crate image;

use cgmath::point::Point;
use image::GenericImage;
use std::num::Bounded;

use scene::Scene;
use light::Light;
use camera::Camera;
use pathtracing::trace_path;

pub fn pixel<T: Camera>(camera: &T, scene: &Scene, x: u32, y: u32) -> image::Rgb<u8> {
    let ray = camera.make_ray(x, y);
    color_from_light(trace_path(scene, ray, 2))
}

pub fn image(width: u32, height: u32, renderer: PixelRenderer) -> image::ImageBuf<image::Rgb<u8>> {
    let mut buffer = image::ImageBuf::new(width, height);
    for y in range(0, height) {
        for x in range(0, width) {
            let pixel = renderer(x, y);
            buffer.put_pixel(x, y, pixel);
        }
    }
    buffer
}

type PixelRenderer<'a> = |u32, u32|:'a -> image::Rgb<u8>;

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

#[cfg(test)]
mod tests {

    extern crate image;

    use image::GenericImage;
    use render;

    use test_helpers::{make_test_scene, make_test_camera};

    #[test]
    fn test_pixel() {
        let scene = make_test_scene();
        let camera = make_test_camera();
        let black = image::Rgb(0, 0, 0);
        assert!(black == render::pixel(&camera, &scene, 1, 1));
        assert!(black == render::pixel(&camera, &scene, 1000, 1));
        assert!(black == render::pixel(&camera, &scene, 1, 1000));
        assert!(black == render::pixel(&camera, &scene, 1000, 1000));
        assert!(black != render::pixel(&camera, &scene, 500, 500));
        assert!(black == render::pixel(&camera, &scene, 500, 124));
        assert!(black != render::pixel(&camera, &scene, 500, 125));
        assert!(black == render::pixel(&camera, &scene, 500, 1000-124));
        assert!(black != render::pixel(&camera, &scene, 500, 1000-125));
    }

    #[test]
    fn test_image() {
        let (width, height) = (100, 100);
        let mut count = 0i;
        let mut imbuf: image::ImageBuf<image::Rgb<u8>>;
        { // We need a scope here because we are borrowing `count`.
            let renderer = |_, _| {count += 1; image::Rgb(0u8, 0, 0)};
            imbuf = render::image(width, height, renderer);
        } // Now we can use `count`.
        assert!(count == 100 * 100);
        assert!(imbuf.get_pixel(34, 21) == image::Rgb(0, 0, 0));
    }
}