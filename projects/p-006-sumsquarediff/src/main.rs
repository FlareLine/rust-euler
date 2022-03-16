#![allow(dead_code)]

fn main() {
	let max_num = 100;
	let (squared_sum, summed_squares) = foreach(max_num);
	let diff = squared_sum - summed_squares;
	println!("{}", diff)
}

fn foreach(max: u32) -> (u32, u32) {
	let mut sum: u32 = 0;
	let mut summed_squares: u32 = 0;
	for i in 0..=max {
		sum += i;
		summed_squares += i * i;
	}
	let squared_sum = sum * sum;
	(squared_sum, summed_squares)
}
