// rustc filename.rs -A warnings
// ./filename


use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
// i - int, u - unsigned, f - float

use std::io::stdin;

fn main()
{
	println!("Hello World!");

	let num = 10;

	// mut - unchangable
	let mut age: i32 = 19;


	// Size of all those datatype
	println!("Max i8 {}", i8::MAX );
	println!("Min i8 {}", i8::MIN );
	println!("Max i16 {}", i16::MAX );
	println!("Min i16 {}", i16::MIN );
	println!("Max i32 {}", i32::MAX );
	println!("Min i32 {}", i32::MIN );
	println!("Max i64 {}", i64::MAX );
	println!("Min i64 {}", i64::MIN );
	println!("Max isize {}", isize::MAX );
	println!("Min isize {}", isize::MIN );
	println!("Max usize {}", usize::MAX );
	println!("Min usize {}", usize::MIN );
	println!("Max f32 {}", f32::MAX );
	println!("Min f32 {}", f32::MIN );
	println!("Max f64 {}", f64::MAX );
	println!("Min f64 {}", f64::MIN );
	

	// bool
	let is_it_true: bool = true;

	let let_x: char = 'x';

	println!("I am {} years old", age);

	// declare 2 variables
	let (f_name, l_name) = ("Michael", "Banas");

}
