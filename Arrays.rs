// rustc filename.rs -A warnings
// ./filename


use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
// i - int, u - unsigned, f - float

use std::io::stdin;

fn main()
{
	let rand_array = [1,2,3];

	println!("{}", rand_array[0]);

	println!("{}", rand_array.len());

	println!("Second 2 : {:?}", &rand_array[1..3]);

}