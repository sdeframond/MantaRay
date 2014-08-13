extern crate image;

use cgmath::point::Point;
use image::GenericImage;
use std::num::Bounded;

use scene::*;
use light::*;
use camera::*;
use pathtracing::*;

pub fn render_pixel<T: RayMaker>(ray_maker: &T, scene: &Scene, x: u32, y: u32) -> image::Rgb<u8> {
    let ray = ray_maker.make_ray(x, y);
    color_from_light(trace_path(scene, ray))
}

pub fn render_image(buffer: &mut image::ImageBuf<image::Rgb<u8>>, render_pixel: PixelRenderer) {
    let (width, height) = buffer.dimensions();
    for y in range(0, height) {
        for x in range(0, width) {
            let pixel = render_pixel(x, y);
            buffer.put_pixel(x, y, pixel);
        }
    }
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

    use render::*;
    use test_helpers::*;

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
}