#![allow(dead_code)]

fn main() {
	let sum = forloop();
	println!("{}", sum);
}

fn forloop() -> i32 {
	let mut sum = 0;
	for i in 0..1000 {
		if i % 3 == 0 || i % 5 == 0 {
			sum += i;
		}
	}
	sum
}

fn iterator() -> i32 {
	(0..1000).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}
