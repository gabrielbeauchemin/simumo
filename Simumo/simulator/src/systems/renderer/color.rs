use image::Rgba;

pub enum Color {
    BLACK,
    GRAY,
    GREEN,
    GREENGRASS,
    GREENFOREST,
    LIGHTGRAY,
    DARKGRAY,
    RED,
    YELLOW,
}

impl Color {
    pub fn get(&self) -> [f32; 4] {
        match self {
            Color::BLACK => [0.0, 0.0, 0.0, 1.0],
            Color::GRAY => [0.5, 0.5, 0.5, 1.0],
            Color::GREEN => [0.0, 1.0, 0.0, 1.0],
            Color::GREENGRASS => [0.8, 0.93, 0.42, 1.0],
            Color::GREENFOREST => [0.2, 0.53, 0.12, 1.0],
            Color::LIGHTGRAY => [0.75, 0.75, 0.75, 1.0],
            Color::DARKGRAY => [0.25, 0.25, 0.25, 1.0],
            Color::RED => [1.0, 0.0, 0.0, 1.0],
            Color::YELLOW => [1.0, 1.0, 0.0, 1.0],
        }
    }
    pub fn r(&self) -> f32 {
        self.get()[0]
    }
    pub fn g(&self) -> f32 {
        self.get()[1]
    }
    pub fn b(&self) -> f32 {
        self.get()[2]
    }
    pub fn a(&self) -> f32 {
        self.get()[3]
    }
    pub fn as_rgba(&self) -> Rgba<u8> {
        Rgba([
            (self.r() * 255.) as u8,
            (self.g() * 255.) as u8,
            (self.b() * 255.) as u8,
            (self.a() * 255.) as u8,
        ])
    }
}
