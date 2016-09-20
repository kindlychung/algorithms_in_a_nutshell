#![feature(stmt_expr_attributes)]

#[macro_use]
pub mod naive_convex_hull;
pub mod greedy_convex_hull;
pub mod definite_num;
pub mod point;
pub mod triangle;
pub mod max_elem;
pub mod needleman_wunsch;
pub mod zm;

#[macro_use]
extern crate itertools;
