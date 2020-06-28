use dim::si::Meter;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::ops::Add;

type Fdef = f64;
type Distance = Meter<Fdef>;

#[derive(Debug)]
pub struct OutOfRange;

impl Error for OutOfRange {}

impl Display for OutOfRange {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Value out of range")
    }
}

const LOWER: Fdef = 0.0;
const UPPER: Fdef = 1.0;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Percentage(Fdef);

impl Percentage {
    pub fn new(value: Fdef) -> Result<Self, OutOfRange> {
        if value >= LOWER && value <= UPPER {
            Ok(Percentage(value))
        } else {
            Err(OutOfRange)
        }
    }

    pub fn new_clamp(value: Fdef) -> Self {
        if value < LOWER {
            Percentage(LOWER)
        } else if value > UPPER {
            Percentage(UPPER)
        } else {
            Percentage(value)
        }
    }

    pub fn half() -> Self {
        Percentage(UPPER / LOWER)
    }

    pub fn lower() -> Self {
        Percentage(LOWER)
    }

    pub fn upper() -> Self {
        Percentage(UPPER)
    }

    pub fn value(self) -> Fdef {
        self.0
    }
}

impl Add for Percentage {
    type Output = Self;

    fn add(self, other: Percentage) -> Self::Output {
        Percentage::new_clamp(self.value() + other.value())
    }
}
