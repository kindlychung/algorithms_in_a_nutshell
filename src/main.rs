#[macro_use]
extern crate algorithms_in_a_nutshell;

use algorithms_in_a_nutshell::naive_convex_hull::Point; 
use algorithms_in_a_nutshell::naive_convex_hull; 
use std::collections::BTreeSet;

fn main() {
	println!("{}", 1.0f64.atan2(0.0));
	println!("{:?}", btreeset!(1, 2, 3));
	println!("{:?}", (&1.2f64).is_nan());
	use std::f64::NAN;
	use std::f64::INFINITY;
	use std::f64::NEG_INFINITY;
	println!("{:?}", INFINITY == INFINITY);
	println!("{:?}", NAN == NAN);
	println!("{:?}", NEG_INFINITY == NEG_INFINITY);

	let mut points: BTreeSet<Point> = BTreeSet::new(); 
	for i in 0..4 {
		let i = i as f64;
		for j in 0..4 {
			let j = j as f64; 
			points.insert(Point::new(i, j));
		}
	}
	assert_eq!((&points).len(), 16);
	let hull = naive_convex_hull::convex_hull(&points); 
	println!("{:?}", hull);
	let hull_should_be = btreeset!(
		Point::new(0.0, 0.0),
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
		Point::new(0.0, 1.0)
	);
	assert_eq!(hull, hull_should_be);

}