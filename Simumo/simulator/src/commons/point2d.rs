use std::ops::{Add, Mul, Sub};

type Fdef = f64;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Point2D {
    pub x: Fdef,
    pub y: Fdef,
}

pub type Vec2D = Point2D;

impl Point2D {
    pub fn new(x: Fdef, y: Fdef) -> Self {
        Point2D { x, y }
    }

    pub fn distance(self, other: Self) -> Fdef {
        let Point2D { x, y } = other - self;
        (x.powi(2) + y.powi(2)).sqrt()
    }
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output::new(self.x + other.x, self.y + other.y)
    }
}

impl Mul<Fdef> for Point2D {
    type Output = Self;

    fn mul(self, amount: Fdef) -> Self {
        Self::Output::new(self.x * amount, self.y * amount)
    }
}
