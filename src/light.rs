use cgmath::point::{Point, Point3};
use cgmath::vector::EuclideanVector;

pub struct LightSource {
    origin: Point3<f32>,
    light: Light
}

impl LightSource {
    pub fn new(o: Point3<f32>, l: Light) -> LightSource {
        LightSource {origin: o, light: l}
    }
    pub fn intensity(&self, point: Point3<f32>) -> Light {
        let d2 = point.sub_p(&self.origin).length2();
        let falloff = 1.0/(1.0 + d2);
        self.light.mul_s(falloff)
    }

    pub fn origin(&self) -> Point3<f32> { self.origin }
}

#[deriving(PartialEq)]
pub struct Light {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

impl Light {
    pub fn new(red: f32, green: f32, blue: f32) -> Light {
        Light {red: red, green: green, blue: blue}
    }

    pub fn add(&self, other: Light) -> Light {
        Light::new(self.red + other.red, self.green + other.green, self.blue + other.blue)
    }

    pub fn mul_l(&self, other: Light) -> Light {
        Light::new(self.red * other.red, self.green * other.green, self.blue * other.blue)
    }

    pub fn mul_s(&self, x: f32) -> Light {
        Light::new(self.red * x, self.green * x, self.blue * x)
    }

    pub fn zero() -> Light {
        Light::new(0.0, 0.0, 0.0)
    }
}