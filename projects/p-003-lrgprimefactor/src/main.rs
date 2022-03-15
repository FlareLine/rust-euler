#![allow(dead_code)]

fn main() {
	let factoid: i64 = 600_851_475_143;
	let largest = iterator(factoid);
	println!("{}", largest);
}

struct Seive {
	current: i64,
}

impl Seive {
	fn new() -> Self {
		Seive {
			current: 2,
		}
	}
}

impl Iterator for Seive {
	type Item = i64;
	fn next(&mut self) -> Option<Self::Item> {
		while !is_prime(self.current as i64) {
			self.current += 1;
		}
		println!("{}", self.current);
		let current = self.current;
		self.current += 1;
		Some(current)
	}
}

fn iterator(factoid: i64) -> i64 {
	let max_factor = (factoid as f64).sqrt().floor();
	Seive::new()
    .take_while(|&i| i <= (max_factor as i64))
    .filter(|&i| factoid % i == 0)
    .last()
    .unwrap_or(0)
}

fn is_prime(n: i64) -> bool {
	if n == 1 {
		return false;
	}

	let mut x: i64 = 2;
	let mut y: i64 = 2;
	let mut d: i64 = 1;

	let g = |i: i64| (i * i + 1) % n;

	while d == 1 {
		x = g(x);
		y = g(g(y));
		d = gcd((x - y).abs(), n);
	}

	d == n
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
	while b != 0 {
		let t = b;
		b = a % b;
		a = t;
	}
	a
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn gcd_hasgcd() {
		let a = 12;
		let b = 8;
		let gcd = gcd(a, b);
		assert_eq!(4, gcd);
	}

	#[test]
	fn gcd_coprime() {
		let a = 13;
		let b = 8;
		let gcd = gcd(a, b);
		assert_eq!(1, gcd);
	}

	#[test]
	fn gcd_zero() {
		let a = 16;
		let b = 0;
		let gcd = gcd(a, b);
		assert_eq!(16, gcd);
	}

	#[test]
	fn isprime_one() {
		let n = 1;
		let is_prime = is_prime(n);
		assert_eq!(false, is_prime);
	}

	#[test]
	fn isprime_two() {
		let n = 2;
		let is_prime = is_prime(n);
		assert_eq!(true, is_prime);
	}

	#[test]
	fn isprime_prime() {
		let n = 5519;
		let is_prime = is_prime(n);
		assert_eq!(true, is_prime);
	}

	#[test]
	fn isprime_notprime() {
		let n = 7903;
		let is_prime = is_prime(n);
		assert_eq!(false, is_prime);
	}
}
