// rustc filename.rs -A warnings
// ./filename


use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
// i - int, u - unsigned, f - float

use std::io::stdin;

fn main()
{
	say_hello("Michael");

	println!("5 + 4 = {}", get_sum(5, 4));

	let sum = get_sum;
	println!("6 + 4 = {}", sum(6, 4));
}

fn say_hello(name: &str)
{
	println!("Hello {}", name);
}

// Receive 2 values
fn get_sum(num1: i32, num2: i32) -> i32
{
	num1 + num2 // u automatically return
}
