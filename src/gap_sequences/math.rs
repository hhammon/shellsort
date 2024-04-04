
pub fn gcd(a: usize, b: usize) -> usize {
	let mut a = a;
	let mut b = b;
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
	fn test_gcd() {
		assert_eq!(gcd(1, 1), 1, "gcd of 1 and 1 is 1");
		assert_eq!(gcd(2, 1), 1, "gcd of 2 and 1 is 1");
		assert_eq!(gcd(1, 2), 1, "gcd of 1 and 2 is 1");
		assert_eq!(gcd(2, 2), 2, "gcd of 2 and 2 is 2");
		assert_eq!(gcd(3, 2), 1, "gcd of 3 and 2 is 1");
		assert_eq!(gcd(2, 3), 1, "gcd of 2 and 3 is 1");
		assert_eq!(gcd(3, 3), 3, "gcd of 3 and 3 is 3");
		assert_eq!(gcd(4, 2), 2, "gcd of 4 and 2 is 2");
		assert_eq!(gcd(2, 4), 2, "gcd of 2 and 4 is 2");
		assert_eq!(gcd(4, 4), 4, "gcd of 4 and 4 is 4");
		assert_eq!(gcd(5, 2), 1, "gcd of 5 and 2 is 1");
		assert_eq!(gcd(2, 5), 1, "gcd of 2 and 5 is 1");
		assert_eq!(gcd(5, 5), 5, "gcd of 5 and 5 is 5");
		assert_eq!(gcd(6, 2), 2, "gcd of 6 and 2 is 2");
		assert_eq!(gcd(2, 6), 2, "gcd of 2 and 6 is 2");
		assert_eq!(gcd(6, 6), 6, "gcd of 6 and 6 is 6");
	}
}