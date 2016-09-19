use std::collections::BTreeSet;
use point::Point;
use triangle::Triangle;

#[macro_export]
macro_rules! btreeset {
    ($($x: expr),*) => {{
         let mut set = ::std::collections::BTreeSet::new();
         $( set.insert($x); )*
         set
    }}
}

pub fn convex_hull(points: &BTreeSet<Point>) -> BTreeSet<Point> {
    // you must have at least 3 points, otherwise there is no hull
    assert!(points.len() >= 3);
    // Remove just one point from the set
    let minus_one = |p: &Point| {
        let mut subset = points.clone();
        subset.remove(p);
        subset
    };
    // set of points that are marked as internal
    let mut p_internal_set = BTreeSet::new();
    // check permutations of 4 points, check if the fourth point is contained in the triangle
    for p_i in points {
        let minus_i = minus_one(&p_i);
        for p_j in minus_i {
            let minus_j = minus_one(&p_j);
            for p_k in minus_j {
                let minus_k = minus_one(&p_k);
                for p_m in minus_k {
                    if Triangle::new(*p_i, p_j, p_k).contains(p_m) {
                        p_internal_set.insert(p_m);
                    }
                }
            }
        }
    }
    // set of points that are not internal
    let mut hull: Vec<_> = points.difference(&p_internal_set).cloned().collect();
    // sort by coordinates so that the first point is the leftmost
    hull.sort();
    let head = hull[0];

    // sort by the angle with the first point
    // when that is equal, sort by distance to head
    hull.sort_by(|a, b| {
        let angle_a = head.angle(&a);
        let angle_b = head.angle(&b);
        angle_a.partial_cmp(&angle_b).unwrap()
    });
    hull.into_iter().collect()
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn test_point() {
        use std::f64::consts::PI;
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(0.0, 1.0);
        assert_eq!(p1.angle(&p2), PI / 2.0);
        assert_eq!(p1.distance(&p2), 1.0);
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(1.0, 1.0);
        assert_eq!(p1.angle(&p2), PI / 4.0);
        assert_eq!(p1.distance(&p2), 2.0f64.sqrt());
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(1.0, -1.0);
        assert_eq!(p1.angle(&p2), -PI / 4.0);
        assert_eq!(p1.distance(&p2), 2.0f64.sqrt());
    }

    #[test]
    fn test_convex_hull_naive() {
        let points: BTreeSet<_> = (0..4)
            .into_iter()
            .flat_map(move |i| {
                let i = i as f64;
                (0..4).into_iter().map(move |j| {
                    let j = j as f64;
                    Point::new(i, j)
                })
            })
            .collect();
        assert_eq!((&points).len(), 16);
        let hull = convex_hull(&points);
        let hull_should_be = btreeset!(Point::new(0.0, 0.0),
                                       Point::new(1.0, 0.0),
                                       Point::new(2.0, 0.0),
                                       Point::new(3.0, 0.0),
                                       Point::new(3.0, 1.0),
                                       Point::new(3.0, 2.0),
                                       Point::new(3.0, 3.0),
                                       Point::new(2.0, 3.0),
                                       Point::new(1.0, 3.0),
                                       Point::new(0.0, 3.0),
                                       Point::new(0.0, 2.0),
                                       Point::new(0.0, 1.0));
        assert_eq!(hull, hull_should_be);
    }

    #[test]
    fn test_triangle() {
        let p2 = Point::new(0.0, 0.0);
        let p1 = Point::new(0.0, 1.0);
        let p0 = Point::new(1.0, 1.0);
        let t = Triangle::new(p0, p1, p2);
        assert_eq!(t.range_x(), (0.0, 1.0));
        assert_eq!(t.range_y(), (0.0, 1.0));
        assert_eq!(t.p0, p2);
        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p0);
        // triangle should not contain its vertices
        assert!(!t.contains(p0));
        assert!(!t.contains(p1));
        assert!(!t.contains(p2));
        // triangle should not contain points on any side
        assert!(!t.contains(Point::new(0.5, 0.5)));
        assert!(!t.contains(Point::new(0.3, 0.3)));
        assert!(!t.contains(Point::new(0.2, 0.2)));
        assert!(!t.contains(Point::new(0.1, 0.1)));
        assert!(!t.contains(Point::new(0.0, 0.1)));
        assert!(!t.contains(Point::new(0.0, 0.2)));
        assert!(!t.contains(Point::new(0.1, 1.0)));
        assert!(!t.contains(Point::new(0.2, 1.0)));
        assert!(!t.contains(Point::new(0.2, 1.1)));
        // strictly inside the triangle
        assert!(t.contains(Point::new(0.5, 0.51)));
        assert!(t.contains(Point::new(0.5, 0.52)));
        assert!(t.contains(Point::new(0.5, 0.53)));
        let p2 = Point::new(0.0, 0.0);
        let p1 = Point::new(0.5, 1.0);
        let p0 = Point::new(1.0, 0.5);
        let t = Triangle::new(p0, p1, p2);
        assert_eq!(t.range_x(), (0.0, 1.0));
        assert_eq!(t.range_y(), (0.0, 1.0));
        assert_eq!(t.p0, p2);
        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p0);
    }
}
