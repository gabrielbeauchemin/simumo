use super::Percentage;
use dim::si::Meter;
use dim::Dimensioned;
use std::cmp::PartialEq;
use std::ops::{Add, AddAssign};

type Fdef = f64;
type Distance = Meter<Fdef>;
const DECIMAL_PRECISION: i32 = 2;

#[derive(Copy, Clone, Debug, PartialOrd)]
pub struct Progress {
    percentage: Percentage,
    total_length: Distance,
}

impl Progress {
    pub fn new(percentage: Percentage, total_length: Distance) -> Self {
        Progress {
            percentage,
            total_length,
        }
    }

    pub fn percentage(&self) -> Percentage {
        self.percentage
    }

    pub fn distance(&self) -> Distance {
        self.percentage().value() * self.total_length
    }
}

impl Add<Distance> for Progress {
    type Output = Self;

    fn add(self, dst: Distance) -> Self::Output {
        let perc = Percentage::new_clamp(distance_to_percentage_float(dst, self.total_length));
        Progress {
            percentage: self.percentage + perc,
            total_length: self.total_length,
        }
    }
}

impl AddAssign<Distance> for Progress {
    fn add_assign(&mut self, dst: Distance) {
        self.percentage = Percentage::new_clamp(
            self.percentage.value() + distance_to_percentage_float(dst, self.total_length),
        );
    }
}

fn distance_to_percentage_float(dst: Distance, total_length: Distance) -> Fdef {
    *(dst / total_length).value_unsafe()
}

impl PartialEq for Progress {
    fn eq(&self, other: &Progress) -> bool {
        approx_eq(self.percentage.value(), other.percentage().value())
            && approx_eq(
                *self.total_length.value_unsafe(),
                *other.total_length.value_unsafe(),
            )
    }
}

fn approx_eq(a: f64, b: f64) -> bool {
    let factor = 10.0f64.powi(DECIMAL_PRECISION);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}
