use std::ops::Sub;

use dim::si::{Meter, M};

use crate::commons::metrics::Fdim;
use crate::commons::Point2D;

lazy_static! {
    static ref EARTH_RADIUS: Fdef = 1000.0 * 6371.0088 * M;
}
const PI: Fdim = std::f64::consts::PI;

///used for modifiability
type Fdef = Meter<Fdim>;
type Angle = Fdim;
type Lon = Angle;
type Lat = Angle;

/// represent a coordinate
/// using the earth's longitude and latitude
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PolarCoord(pub Lat, pub Lon);

impl PolarCoord {
    pub fn from_cartesian(coord: &CartesianCoord) -> Self {
        Self::from_float(
            (coord.y / *EARTH_RADIUS).to_degrees(),
            (coord.x / *EARTH_RADIUS).to_degrees(),
        )
    }

    pub fn from_float(lat: Fdim, lon: Fdim) -> Self {
        Self(lat, lon)
    }

    pub fn from_point(Point2D { x, y }: &Point2D) -> Self {
        Self::from_float(*y, *x)
    }
}

impl Sub for PolarCoord {
    type Output = PolarCoord;

    fn sub(self, other: PolarCoord) -> Self::Output {
        PolarCoord(self.0 - other.0, self.1 - other.1)
    }
}

/// represent a coordinate
/// using a flat X and Y surface
///
#[derive(Clone, Debug)]
pub struct CartesianCoord {
    pub x: Fdef,
    pub y: Fdef,
}

impl CartesianCoord {
    pub fn new(x: Fdef, y: Fdef) -> Self {
        Self { x, y }
    }

    pub fn from_polar(coord: &PolarCoord) -> Self {
        Self::new(
            coord.1.to_radians() * *EARTH_RADIUS,
            coord.0.to_radians() * *EARTH_RADIUS,
        )
    }

    pub fn from_float(x: Fdim, y: Fdim) -> Self {
        Self::new(Fdef::new(x), Fdef::new(y))
    }

    pub fn from_point(Point2D { x, y }: &Point2D) -> Self {
        Self::from_float(*x, *y)
    }
}

impl Sub for CartesianCoord {
    type Output = CartesianCoord;
    fn sub(self, other: CartesianCoord) -> Self::Output {
        CartesianCoord::new(self.x - other.x, self.y - other.y)
    }
}

impl Default for CartesianCoord {
    fn default() -> Self {
        Self::from_float(0.0, 0.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn polar_to_cartesian() {
        let pcoord = PolarCoord::from_float(45.0, 90.0);
        let ccoord = CartesianCoord::from_polar(&pcoord);

        assert_eq!(10007557.221017962 * M, ccoord.x);
        assert_eq!(5003778.610508981 * M, ccoord.y);
    }

    #[test]
    fn cartesian_to_polar() {
        let ccoord = CartesianCoord::from_float(10007557.221017962, 5003778.610508981);
        let pcoord = PolarCoord::from_cartesian(&ccoord);
        assert_eq!(45., pcoord.0.floor());
        assert_eq!(90., pcoord.1.floor());
    }

    #[test]
    fn polar_to_cartesian_30_90() {
        let pcoord = PolarCoord(30.0, 90.0);
        let ccoord = CartesianCoord::from_polar(&pcoord);

        assert_eq!(10007557.221017962 * M, ccoord.x);
        assert_eq!(3335852.407005987 * M, ccoord.y);
    }
}
