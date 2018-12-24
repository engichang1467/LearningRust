// rustc filename.rs -A warnings
// ./filename


use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
// i - int, u - unsigned, f - float

use std::io::stdin;

// Closure - blocks of code & accept parameter

fn main()
{
	let sum_nums = |x: i32, y: i32| x + y;
	println!("7 + 8 = {}", sum_nums(7,8));

	let num_ten = 10;

	let add_10 = |x: i32| x + num_ten;
	println!("5 + 10 = {}", add_10(5));

}