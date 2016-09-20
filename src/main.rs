#[macro_use]
extern crate algorithms_in_a_nutshell;

use algorithms_in_a_nutshell::point::Point; 
use algorithms_in_a_nutshell::naive_convex_hull; 
use std::collections::BTreeSet;
use algorithms_in_a_nutshell::needleman_wunsch::NeedlemanWunsch;

fn main() {
		fn similarity(c1: &char, c2: &char) -> i64 {
			if c1 == c2 {
				1
			} else {
				-1
			}
		}
		let mut vec1: Vec<char>  = "what".chars().collect();
		let mut vec2: Vec<char>  = "white".chars().collect();
		let res = vec1.align(&mut vec2, &similarity, -1);
		let mut s0 = String::new();
		let mut s1 = String::new();
		use std::fmt::Write;
		for x in res.0 {
			match x {
				Some(c) => write!(&mut s0, "{}", c).unwrap(),
				_ => write!(&mut s0, "{}", '_').unwrap(),
			}
		}
		for x in res.1 {
			match x {
				Some(c) => write!(&mut s1, "{}", c).unwrap(),
				_ => write!(&mut s1, "{}", '_').unwrap(),
			}
		}
		println!("{}", s0);
		println!("{}", s1);
}