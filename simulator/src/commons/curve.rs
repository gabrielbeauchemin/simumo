use super::percentage::Percentage;
use super::point2d::Point2D;
use super::Progress;
use dim::si::Meter;
use dim::Dimensioned;

type Fdef = f64;
type Distance = Meter<Fdef>;

trait Lerp {
    fn lerp(a: Self, b: Self, t: Fdef) -> Self;
}

impl Lerp for Fdef {
    fn lerp(a: Self, b: Self, t: Fdef) -> Self {
        a * (1.0 - t) + b * t
    }
}

impl Lerp for Point2D {
    fn lerp(a: Self, b: Self, t: Fdef) -> Self {
        a + ((b - a) * t)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CurvePoint {
    progress: Progress,
    point: Point2D,
}

impl CurvePoint {
    fn new(progress: Progress, point: Point2D) -> Self {
        CurvePoint { progress, point }
    }

    pub fn progress(&self) -> Progress {
        self.progress
    }

    pub fn point(&self) -> Point2D {
        self.point
    }
}

#[derive(Clone, Debug)]
pub struct Curve {
    length: Distance,
    points: Vec<CurvePoint>,
}

impl Curve {
    pub fn new(points: Vec<Point2D>) -> Self {
        assert!(points.len() >= 2);

        let length = Distance::new(get_total_length(&points));

        let mut points: Vec<_> = points
            .iter()
            .map(|p| CurvePoint::new(Progress::new(Percentage::lower(), length), *p))
            .collect();

        let mut acc = 0.0;
        for i in 1..points.len() {
            acc += points[i].point().distance(points[i - 1].point());
            points[i].progress += Distance::new(acc);
        }

        Curve { length, points }
    }

    pub fn length(&self) -> Distance {
        self.length
    }

    pub fn distance_to_progress(&self, distance: Distance) -> Progress {
        let p = distance / self.length();
        let percentage = Percentage::new_clamp(*p.value_unsafe());
        Progress::new(percentage, self.length())
    }

    pub fn percentage_to_progress(&self, percentage: Percentage) -> Progress {
        Progress::new(percentage, self.length())
    }

    pub fn get_location_at_distance(&self, distance: Distance) -> CurvePoint {
        let perc = self.distance_to_progress(distance);
        self.get_location_at_percentage(perc.percentage())
    }

    pub fn get_location_at_percentage(&self, percentage: Percentage) -> CurvePoint {
        let num_points = self.points.len();
        let last_i = num_points - 1;

        let i = match self.get_point_index_for_input_value(percentage) {
            None => return *self.points.first().unwrap(),
            Some(i) => {
                if i == last_i {
                    if !is_looped(&self.points) {
                        return self.points[last_i];
                    } else if percentage >= self.points[last_i].progress().percentage() {
                        return *self.points.first().unwrap();
                    }
                }
                i
            }
        };

        let is_loop_segment = is_looped(&self.points) && i == last_i;
        let next_i = if is_loop_segment { 0 } else { i + 1 };

        let prev = &self.points[i];
        let next = &self.points[next_i];

        let diff = if is_loop_segment {
            0.0
        } else {
            next.progress().percentage().value() - prev.progress().percentage().value()
        };

        if diff > 0.0 {
            let alpha = (percentage.value() - prev.progress().percentage().value()) / diff;
            let point = Lerp::lerp(prev.point(), next.point(), alpha);
            let distance = percentage.value() * self.length();
            CurvePoint::new(Progress::new(percentage, distance), point)
        } else {
            self.points[i]
        }
    }

    fn get_point_index_for_input_value(&self, percentage: Percentage) -> Option<usize> {
        let num_points = self.points.len();
        let last_i = num_points - 1;

        if percentage < self.points.first().unwrap().progress().percentage() {
            return None;
        }

        if percentage >= self.points[last_i].progress().percentage() {
            return Some(last_i);
        }

        let mut min_i = 0;
        let mut max_i = num_points;

        while max_i - min_i > 1 {
            let mid = (min_i + max_i) / 2;

            if self.points[mid].progress().percentage() <= percentage {
                min_i = mid;
            } else {
                max_i = mid;
            }
        }

        Some(min_i)
    }

    fn get_segment_length(&self, i: usize, param: Fdef) -> Fdef {
        let p0 = self.points[i].point();
        let p1 = if i == self.points.len() - 1 {
            self.points[0].point()
        } else {
            self.points[i + 1].point()
        };

        p1.distance(p0) * param
    }
}

fn is_looped<T: PartialEq>(points: &[T]) -> bool {
    if points.len() < 2 {
        return false;
    }
    points.first().unwrap() == points.last().unwrap()
}

fn get_total_length(points: &[Point2D]) -> Fdef {
    let mut acc = 0.0f64;
    for i in 1..points.len() {
        acc += points[i].distance(points[i - 1]);
    }
    acc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn over_distance_gives_final_point() {
        let line = Curve::new(vec![Point2D::new(0.0, 0.0), Point2D::new(10.0, 15.0)]);

        assert_eq!(
            line.get_location_at_percentage(Percentage::new(1.0).unwrap()),
            CurvePoint::new(
                Progress::new(Percentage::new_clamp(1.0), Distance::new(18.02)),
                Point2D::new(10.0, 15.0)
            )
        );
    }

    #[test]
    fn mid_distance_gives_mid_points() {
        let line = Curve::new(vec![Point2D::new(0.0, 0.0), Point2D::new(3.0, 4.0)]);

        assert_eq!(
            line.get_location_at_percentage(Percentage::new(0.5).unwrap()),
            CurvePoint::new(
                Progress::new(Percentage::new_clamp(0.5), Distance::new(2.5)),
                Point2D::new(1.5, 2.0)
            )
        );
    }
}
