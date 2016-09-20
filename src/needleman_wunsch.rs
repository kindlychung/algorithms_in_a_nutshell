
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Match, // go diagonally, left and up
    Left,
    Up,
    Undefined, 
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ResultEntry {
    pub score: i64,
    pub direction: Direction,
}

impl ResultEntry {
	fn new(s: i64, d: Direction) -> ResultEntry {
		ResultEntry {
			score: s, 
			direction: d,
		}
	}
}

pub trait NeedlemanWunsch<T> {
    fn align(&mut self, vec: &mut Vec<T>, similarity: &Fn(&T, &T) -> i64, gap_penalty: i64) -> (Vec<Option<T>>, Vec<Option<T>>);
}


impl<T> NeedlemanWunsch<T> for Vec<T> where T: Eq + Copy + Clone {
	fn align(&mut self, vec: &mut Vec<T>, similarity: &Fn(&T, &T) -> i64, gap_penalty: i64) -> (Vec<Option<T>>, Vec<Option<T>>) {
		let mut mat = init_align_matrix(self.len() , vec.len());
		fill_align_matrix(&mut mat, self, vec, similarity, gap_penalty);
		let mat = &mat; // no need to mutate mat any more. 
		let mut vec1: Vec<Option<T>> = Vec::new(); 
		let mut vec2: Vec<Option<T>> = Vec::new(); 
		// the dimension of the alignment matrix is one larger than vec1.len by vec2.len
		let mut i = self.len(); 
		let mut j = vec.len(); 
		loop {
			print_mat(mat);
//			println!("Node: {:?}, Node direction: {:?}", (i, j), mat[i][j].direction);
			match mat[i][j].direction {
				Direction::Match => {
					// the dimension of the alignment matrix is one larger than vec1.len by vec2.len
//					println!("Tracing {:?}", (i, j));
					vec1.insert(0, Some(self[i-1]));
					vec2.insert(0, Some(vec[j-1]));
					i -= 1; 
					j -= 1; 
				}, 
				Direction::Left => {
					vec1.insert(0, None);
					// the dimension of the alignment matrix is one larger than vec1.len by vec2.len
					vec2.insert(0, Some(vec[j-1]));
					j -= 1; 
				}, 
				Direction::Up => {
					// the dimension of the alignment matrix is one larger than vec1.len by vec2.len
					vec1.insert(0, Some(self[i-1]));
					vec2.insert(0, None);
					i -= 1;
				}, 
				Direction::Undefined => {
					break;
				}
			}
		}
		return (vec1, vec2);
	}
}

#[test]
fn test_align() {
    fn similarity(c1: &char, c2: &char) -> i64 {
        if c1 == c2 { 1 } else { -1 }
    }
    let mut vec1: Vec<char> = "what".chars().collect();
    let mut vec2: Vec<char> = "white".chars().collect();
    let res = vec1.align(&mut vec2, &similarity, -1);
	let res_should_be = (
		vec![Some('w'), Some('h'), Some('a'), Some('t'), None], 
		vec![Some('w'), Some('h'), Some('i'), Some('t'), Some('e')], 
	);
	assert_eq!(res, res_should_be);
    let mut vec1: Vec<char> = "ab".chars().collect();
    let mut vec2: Vec<char> = "aeb".chars().collect();
    let res = vec1.align(&mut vec2, &similarity, -1);
	let res_should_be = (
		vec![Some('a'), None, Some('b')], 
		vec![Some('a'), Some('e'), Some('b')], 
	);
	assert_eq!(res, res_should_be);
}

type AlignMatrix = Vec<Vec<ResultEntry>>;

/// len1: Length of the first vector, or number of rows for the alignment matrix.
/// len2: Length of the second vector, or number of cols for the alignment matrix.
fn init_align_matrix(len1: usize, len2: usize) -> AlignMatrix {
	let len1 = len1 + 1; 
	let len2 = len2 + 1; 
	// starting point has undefined direction
	let mut mat: AlignMatrix = vec![vec![ResultEntry::new(0, Direction::Undefined); len2]; len1];
	// first row all go left
	for i in 1..len2 {
		mat[0][i] = ResultEntry::new(-(i as i64), Direction::Left);
	}
	// first col all go up
	for i in 1..len1 {
		mat[i][0] = ResultEntry::new(-(i as i64), Direction::Up);
	}
	mat[0][0] = ResultEntry::new(0, Direction::Undefined);
	mat
}

#[test]
fn test_align_mat() {
	let m = init_align_matrix(2, 3); 
	let m_should_be = vec![
		vec![ResultEntry::new( 0, Direction::Undefined), ResultEntry::new(-1, Direction::Left),      ResultEntry::new(-2, Direction::Left),      ResultEntry::new(-3, Direction::Left)],
		vec![ResultEntry::new(-1, Direction::Up),        ResultEntry::new( 0, Direction::Undefined), ResultEntry::new( 0, Direction::Undefined), ResultEntry::new( 0, Direction::Undefined)],
		vec![ResultEntry::new(-2, Direction::Up),        ResultEntry::new( 0, Direction::Undefined), ResultEntry::new( 0, Direction::Undefined), ResultEntry::new( 0, Direction::Undefined)],
	];
	assert_eq!(m, m_should_be);
}

#[test]
fn test_fill_mat() {
		fn similarity(c1: &char, c2: &char) -> i64 {
			if c1 == c2 {
				1
			} else {
				-1
			}
		}
		let mut vec1: Vec<char>  = "ab".chars().collect();
		let mut vec2: Vec<char>  = "aeb".chars().collect();
		let mut mat = init_align_matrix(vec1.len(), vec2.len());
		fill_align_matrix(&mut mat, &mut vec1, &mut vec2, &similarity, -1);
		use self::Direction::Undefined as ud;
		use self::Direction::Left as lt; 
		use self::Direction::Up as up; 
		use self::Direction::Match as mt;
		let res_should_be = vec![
			vec![ResultEntry::new(0, ud), ResultEntry::new(-1, lt), ResultEntry::new(-2, lt), ResultEntry::new(-3, lt)],
			vec![ResultEntry::new(-1, up), ResultEntry::new(1, mt), ResultEntry::new(0, lt), ResultEntry::new(-1, lt)],
			vec![ResultEntry::new(-2, up), ResultEntry::new(0, up), ResultEntry::new(0, mt), ResultEntry::new(1, mt)],
		];
		assert_eq!(mat, res_should_be);
}

fn fill_align_matrix<T>(mat: &mut AlignMatrix, vec1: &mut Vec<T>, vec2: &mut Vec<T>, similarity: &Fn(&T, &T) -> i64, gap_penalty: i64) {
	assert_eq!(mat.len(), vec1.len() + 1);
	assert_eq!(mat[0].len(), vec2.len() + 1);
	
	fn get_score_set_direction<U>(mat: &mut AlignMatrix, vec1: &Vec<U>, vec2: &Vec<U>, similarity: &Fn(&U, &U) -> i64, gap_penalty: i64, i: usize, j: usize) -> i64 {
		if i == 0 || j == 0 {
			mat[i][j].score
		} else if mat[i][j].direction != Direction::Undefined {
			mat[i][j].score
		} else {
			let up_left = get_score_set_direction(mat, vec1, vec2, similarity, gap_penalty, i - 1, j - 1);
			let up      = get_score_set_direction(mat, vec1, vec2, similarity, gap_penalty, i - 1, j);
			let left    = get_score_set_direction(mat, vec1, vec2, similarity, gap_penalty, i, j - 1);
			// the dimension of the alignment matrix is one larger than vec1.len by vec2.len
			let similarity_score = similarity(&vec1[i-1], &vec2[j-1]); 
			let match_score =  up_left + similarity_score; 
			let up_score = up + gap_penalty; 
			let left_score = left + gap_penalty;
			let m = *[match_score, up_score, left_score].iter().max().unwrap();
//			print_mat(mat);
//			println!("Node: {:?}", (i, j));
//			println!("up_left, up, left: {:?}", (up_left, up, left));
//			println!("Match, up, left: {:?}", (match_score, up_score, left_score));
//			println!("Max score: {}", m);
			if m == match_score {
//				println!("Go Up and Left!");	
				mat[i][j] = ResultEntry::new(m, Direction::Match);
			} else if m == up_score {
				mat[i][j] = ResultEntry::new(m, Direction::Up);
//				println!("Go Up!");	
			} else {
//				println!("Go Left!");	
				mat[i][j] = ResultEntry::new(m, Direction::Left);
			}
			m
		}
	}
	for i in 1..(vec1.len() + 1) {
		for j in 1..(vec2.len() + 1) {
			get_score_set_direction(mat, vec1, vec2, similarity, gap_penalty, i, j);
		}
	}
}

fn print_mat(mat: &AlignMatrix) {
	use std::fmt::Write; 
	fn direction_to_arrow(d: Direction) -> char {
		match d {
			Direction::Match => '↖', 
			Direction::Up => '↑', 
			Direction::Left => '←', 
			Direction::Undefined => '.', 
		}
	}
	for row in mat {
		let mut s = String::new(); 
		for el in row { 
			write!(&mut s, "{:>4}", el.score).unwrap(); 
		} 
		println!("{}", s);
	}
	for row in mat {
		let mut s = String::new(); 
		for el in row { 
			write!(&mut s, "{:>4}", direction_to_arrow(el.direction)).unwrap(); 
		} 
		println!("{}", s);
	}
}

