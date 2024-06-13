use std::cmp::Ordering;

use leptos::Signal;

pub fn make_hull(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut points = points.to_vec();
    points.sort_by(|a, b| point_comparator(a, b));
    make_hull_presorted(&points)
}

pub fn point_comparator(a: &(f64, f64), b: &(f64, f64)) -> Ordering {
    if a.0 < b.0 {
        Ordering::Less
    } else if a.0 > b.0 {
        Ordering::Greater
    } else if a.1 < b.1 {
        Ordering::Less
    } else if a.1 > b.1 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub type ElPoints = (Signal<f64>, Signal<f64>, Signal<f64>, Signal<f64>);

pub fn get_points_from_el((top, right, bottom, left): &ElPoints) -> Vec<(f64, f64)> {
    vec![
        (left(), top()),
        (right(), top()),
        (right(), bottom()),
        (left(), bottom()),
    ]
}

pub fn make_hull_from_elements(els: &[ElPoints]) -> Vec<(f64, f64)> {
    let points = els
        .iter()
        .flat_map(|el| get_points_from_el(el))
        .collect::<Vec<(f64, f64)>>();
    make_hull(&points)
}

/// Returns the convex hull, assuming that each points[i] <= points[i + 1]. Runs in O(n) time.
pub fn make_hull_presorted(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut lower: Vec<(f64, f64)> = Vec::new();
    for p in points {
        while lower.len() >= 2 {
            let last = lower.len() - 1;
            let second_last = lower.len() - 2;
            let v1 = (
                lower[last].0 - lower[second_last].0,
                lower[last].1 - lower[second_last].1,
            );
            let v2 = (p.0 - lower[last].0, p.1 - lower[last].1);
            if v1.0 * v2.1 - v1.1 * v2.0 < 0.0 {
                lower.pop();
            } else {
                break;
            }
        }
        lower.push(*p);
    }
    let mut upper: Vec<(f64, f64)> = Vec::new();
    for p in points.iter().rev() {
        while upper.len() >= 2 {
            let last = upper.len() - 1;
            let second_last = upper.len() - 2;
            let v1 = (
                upper[last].0 - upper[second_last].0,
                upper[last].1 - upper[second_last].1,
            );
            let v2 = (p.0 - upper[last].0, p.1 - upper[last].1);
            if v1.0 * v2.1 - v1.1 * v2.0 < 0.0 {
                upper.pop();
            } else {
                break;
            }
        }
        upper.push(*p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

pub fn point_in_polygon(point: (f64, f64), polygon: &[(f64, f64)]) -> bool {
    let mut result = false;
    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        let xi = polygon[i].0;
        let yi = polygon[i].1;
        let xj = polygon[j].0;
        let yj = polygon[j].1;

        if (yi > point.1) != (yj > point.1) && point.0 < (xj - xi) * (point.1 - yi) / (yj - yi) + xi
        {
            result = !result;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_hull() {
        let points = vec![
            (0.0, 0.0),
            (0.0, 4.0),
            (-4.0, 0.0),
            (5.0, 0.0),
            (0.0, -6.0),
            (1.0, 0.0),
        ];
        let hull = make_hull(&points);
        assert_eq!(hull, vec![(-4.0, 0.0), (0.0, -6.0), (5.0, 0.0), (0.0, 4.0)]);
    }

    #[test]
    fn test_point_in_polygon() {
        let polygon = vec![(0.0, 0.0), (0.0, 4.0), (4.0, 4.0), (4.0, 0.0)];
        assert_eq!(point_in_polygon((2.0, 2.0), &polygon), true);
        assert_eq!(point_in_polygon((0.0, 0.0), &polygon), true);
        assert_eq!(point_in_polygon((1.0, 1.0), &polygon), true);
        assert_eq!(point_in_polygon((1.1, 3.9), &polygon), true);
        assert_eq!(point_in_polygon((3.9, 1.1), &polygon), true);
        assert_eq!(point_in_polygon((-0.1, 0.0), &polygon), false);
        assert_eq!(point_in_polygon((0.0, 4.0), &polygon), false);
        assert_eq!(point_in_polygon((4.0, 2.0), &polygon), false);
        assert_eq!(point_in_polygon((2.0, 4.0), &polygon), false);
        assert_eq!(point_in_polygon((2.0, 2.0), &vec![]), false);
    }
}
