
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
	fn test_gcd_1_1() {
		assert_eq!(gcd(1, 1), 1);
	}

	#[test]
	fn test_gcd_2_1() {
		assert_eq!(gcd(2, 1), 1);
	}

	#[test]
	fn test_gcd_1_2() {
		assert_eq!(gcd(1, 2), 1);
	}

	#[test]
	fn test_gcd_2_2() {
		assert_eq!(gcd(2, 2), 2);
	}

	#[test]
	fn test_gcd_3_2() {
		assert_eq!(gcd(3, 2), 1);
	}

	#[test]
	fn test_gcd_2_3() {
		assert_eq!(gcd(2, 3), 1);
	}

	#[test]
	fn test_gcd_3_3() {
		assert_eq!(gcd(3, 3), 3);
	}

	#[test]
	fn test_gcd_4_2() {
		assert_eq!(gcd(4, 2), 2);
	}

	#[test]
	fn test_gcd_2_4() {
		assert_eq!(gcd(2, 4), 2);
	}

	#[test]
	fn test_gcd_4_4() {
		assert_eq!(gcd(4, 4), 4);
	}

	#[test]
	fn test_gcd_5_2() {
		assert_eq!(gcd(5, 2), 1);
	}

	#[test]
	fn test_gcd_2_5() {
		assert_eq!(gcd(2, 5), 1);
	}

	#[test]
	fn test_gcd_5_5() {
		assert_eq!(gcd(5, 5), 5);
	}

	#[test]
	fn test_gcd_6_2() {
		assert_eq!(gcd(6, 2), 2);
	}

	#[test]
	fn test_gcd_2_6() {
		assert_eq!(gcd(2, 6), 2);
	}

	#[test]
	fn test_gcd_6_6() {
		assert_eq!(gcd(6, 6), 6);
	}
}