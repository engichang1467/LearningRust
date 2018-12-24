// rustc filename.rs -A warnings
// ./filename


use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
// i - int, u - unsigned, f - float

use std::io::stdin;

fn main()
{
	let rand_tuple = ("Mikey", 40);

	let rand_tuple2: (&str, i8) = ("Mikey", 40);

	println!("Name : {}", rand_tuple2.0);

}