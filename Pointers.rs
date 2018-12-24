// rustc filename.rs -A warnings
// ./filename


use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
// i - int, u - unsigned, f - float

use std::io::stdin;

fn main()
{
	let prim_val = 1;
	let prim_val2 = prim_val;
	println!("prim_val : {}", prim_val);

	let vect2 = vec![1,2,3];

	println!("Sum of Vect : {}", sum_vects(&vect2));

	println!("Vect : {:?}", vect2);
}

fn sum_vects(v1: &Vec<i32>) -> i32
{
	let sum = v1.iter().fold(0, |mut sum, x| {sum += x; sum});

	return sum;
}