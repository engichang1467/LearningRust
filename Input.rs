// rustc filename.rs -A warnings
// ./filename


use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
// i - int, u - unsigned, f - float

use std::io::stdin;

fn main()
{
	'outer: loop 
	{
		let number: i32 = 10;
		println!("Pick a Number");

		loop 
		{
			let mut line = String::new();

			let input = stdin().read_line(&mut line);

			let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

			match guess 
			{
				None => println!("Enter a Number"),
				Some(n) if n == number  => 
				{
					println!("You Guessed it");
					break 'outer;
				}
				Some(n) if n < number =>
				println!("Too low"),
				Some(n) if n > number =>
				println!("Too High"),
				Some(_) => println!("Error")
				
			}
		}


	}

}