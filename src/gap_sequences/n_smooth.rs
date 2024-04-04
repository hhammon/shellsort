
pub fn list_n_smooth(n: usize, max: usize) -> Vec<usize> {
	let mut sieve: Vec<u8> = vec![0; (max >> 2) + 1];

	//This is basically an extension to the sieve of Eratosthenes
	//Instead of a single bit per value, we use 2 bits per value
	//The right bit will be 0 if the value is prime and 1 otherwise
	//The left bit will be 0 if the value is n-smooth and 1 otherwise

	const COMPOSITE_MASK: u8 = 1;
	const NONSMOOTH_MASK: u8 = 2;
	const COMPOSITE_NONSMOOTH_MASK: u8 = COMPOSITE_MASK | NONSMOOTH_MASK;

	for i in 2..=max {
		let byte_index = i >> 2;
		let bit_index = (i & 3) * 2;
		let mask = COMPOSITE_MASK << bit_index;
		if sieve[byte_index] & mask == 0 {
			if i <= n {
				//We only need to mark multiples composite
				let mut j = i * i;
				while j <= max {
					let byte_index = j >> 2;
					let bit_index = (j & 3) * 2;
					let mask = COMPOSITE_MASK << bit_index;
					sieve[byte_index] |= mask;
					j += i;
				}
			} else {
				//We need to mark i not smooth
				//And multiples neither smooth nor prime
				let mask = NONSMOOTH_MASK << bit_index;
				sieve[byte_index] |= mask;

				let mut j = i * 2;
				while j <= max {
					let byte_index = j >> 2;
					let bit_index = (j & 3) * 2;
					let mask = COMPOSITE_NONSMOOTH_MASK << bit_index;
					sieve[byte_index] |= mask;
					j += i;
				}
			}
		}
	}

	let mut smooths: Vec<usize> = Vec::new();
	for i in 1..=max {
		let byte_index = i >> 2;
		let bit_index = (i & 3) * 2;
		let mask = NONSMOOTH_MASK << bit_index;
		if sieve[byte_index] & mask == 0 {
			smooths.push(i);
		}
	}

	smooths
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_list_n_smooth_1() {
		let smooths = list_n_smooth(1, 1000);
		assert_eq!(smooths, vec![1]);
	}

	#[test]
	fn test_list_n_smooth_2() {
		let smooths = list_n_smooth(2, 1000);
		assert_eq!(smooths, vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512]);
	}

	#[test]
	fn test_list_n_smooth_3() {
		let smooths = list_n_smooth(3, 1000);
		assert_eq!(smooths, vec![
			1, 2, 3, 4, 6, 8, 9, 12, 16, 18, 24, 27, 32, 36, 48, 54, 64, 
			72, 81, 96, 108, 128, 144, 162, 192, 216, 243, 256, 288, 324, 
			384, 432, 486, 512, 576, 648, 729, 768, 864, 972
		]);
	}

	#[test]
	fn test_list_n_smooth_4() {
		let smooths = list_n_smooth(4, 1000);
		assert_eq!(smooths, vec![
			1, 2, 3, 4, 6, 8, 9, 12, 16, 18, 24, 27, 32, 36, 48, 54, 64, 
			72, 81, 96, 108, 128, 144, 162, 192, 216, 243, 256, 288, 324, 
			384, 432, 486, 512, 576, 648, 729, 768, 864, 972
		]);
	}

	#[test]
	fn test_list_n_smooth_5() {
		let smooths = list_n_smooth(5, 400);
		assert_eq!(smooths, vec![
			1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 25, 27, 30, 
			32, 36, 40, 45, 48, 50, 54, 60, 64, 72, 75, 80, 81, 90, 96, 100, 
			108, 120, 125, 128, 135, 144, 150, 160, 162, 180, 192, 200, 216, 
			225, 240, 243, 250, 256, 270, 288, 300, 320, 324, 360, 375, 384, 
			400
		]);
	}
}