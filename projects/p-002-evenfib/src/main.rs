#![allow(dead_code)]

fn main() {
	let sum = forloop();
	println!("{}", sum);
}

fn forloop() -> u32 {
	let mut sum = 0;
	let mut prev = 1;
	let mut current = 2;
	while current < 4000000 {
		if current % 2 == 0 {
			sum += current;
		}
		let next = prev + current;
		prev = current;
		current = next;
	}
	sum
}

struct FibNum {
	prev: u32,
	current: u32,
}

impl FibNum {
	fn new() -> Self {
		FibNum {
			prev: 1,
			current: 2,
		}
	}
}

impl Iterator for FibNum {
	type Item = u32;
	fn next(&mut self) -> Option<Self::Item> {
		let sum = self.prev + self.current;
		self.prev = self.current;
		self.current = sum;
		Some(self.prev)
	}
}

fn iterator() -> u32 {
	FibNum::new()
    .filter(|i| i % 2 == 0)
    .take_while(|i| *i <= 4_000_000)
		.sum()
}
