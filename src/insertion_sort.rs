pub fn insertion_sort<T>(vec: &mut Vec<T>) where T: Ord + Copy {
	fn insert<U>(vec: &mut Vec<U>, pos: usize, value: U) where U: Ord + Copy {
		assert!(pos > 0);
		let mut pos: usize = pos - 1;
		loop {
			let value_at_pos = vec[pos]; 
			if value_at_pos <= value {
				break;
			}
			vec[pos + 1] = value_at_pos; 
			if pos == 0 {
				vec[pos] = value;
				return ();
			}
			pos -= 1;
		}
		vec[pos + 1] = value;
	}
	for i in 1..vec.len() {
		let value = vec[i];
		insert(vec, i, value);
	}
}

#[test]
fn test_insertion_sort() {
	let mut vec = vec![9, 8, 7, 11, 10]; 
	insertion_sort(&mut vec);
	let vec_res: Vec<_> = (7..12).collect();
	assert_eq!(vec, vec_res);
}
