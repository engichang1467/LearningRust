// rustc filename.rs -A warnings
// ./filename


use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
// i - int, u - unsigned, f - float

use std::io::stdin;

fn main()
{
	let rand_string = "I am a random string";

	// How to split ur strings
	/*
	println!("Length : {}", rand_string.len());

	let (first, second) = rand_string.split_at(6);
	println!("Fist : {}, Second : {}", first, second);
	*/
	// make a iterator for our string
	let count = rand_string.chars().count();
	let mut chars = rand_string.chars();

	let mut indiv_char = chars.next();

	loop 
	{
		match indiv_char 
		{
			Some(x) => println!("{}", x),
			None => break,
		}
		indiv_char = chars.next();
	}

	let mut iter = rand_string.split_whitespace();

	let mut indiv_word = iter.next();

	loop 
	{
		match indiv_word 
		{
			Some(x) => println!("{}", x),
			None => break,
		}
		indiv_word = iter.next();
	}

	// to iterate over the line of the string
	let rand_string2 = "I am a random string\nThere are other strings like it\nThis string is the best";

	let mut lines = rand_string2.lines();
	let mut indiv_line = lines.next();

	loop 
	{
	 	match indiv_line 
	 	{
	 		Some(x) => println!("{}", x),
	 		None => break,
	 	}
	 	indiv_line = lines.next();
	 } 

	 println!("Find Best : {}", rand_string2.contains("best"));
}