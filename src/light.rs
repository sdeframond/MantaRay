use cgmath::point::Point3;

pub struct LightSource {
    origin: Point3<f32>,
    light: Light
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
}