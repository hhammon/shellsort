use std::num::ParseIntError;
pub enum GapSequence {
    Ciura,
    Tokuda,
    Custom(Vec<usize>),
}

impl GapSequence {
    pub fn to_vec(&self, array_size: usize) -> Vec<usize> {
        match self {
            Self::Ciura => ciura(),
            Self::Tokuda => tokuda(array_size),
            Self::Custom(gaps) => gaps.clone(),
        }
    }
}

impl TryFrom<String> for GapSequence {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "ciura" => Ok(Self::Ciura),
            "tokuda" | "" => Ok(Self::Tokuda),
            _ => {
                let gaps: Result<Vec<usize>, ParseIntError> = value
                    .split(",")
                    .map(|s| s.trim().parse::<usize>())
                    .collect::<>();

               match gaps {
                   Ok(gaps) => Ok(Self::Custom(gaps)),
                   Err(e) => Err(e),
               }
            }
        }
    }
}

/// Returns the Ciura gap sequence.
/// 
/// [OIES A102549](https://oeis.org/A102549)
pub fn ciura() -> Vec<usize> {
	vec![1, 4, 10, 23, 57, 132, 301, 701, 1750]
}

/// Returns the Tokuda gap sequence.
/// 
/// ceil(((9/4)^k - 1) / (9/4 - 1))\
/// k begins at 1.
/// 
/// [OIES A108870](https://oeis.org/A108870)
pub fn tokuda(array_size: usize) -> Vec<usize> {
	let mut gaps: Vec<usize> = vec![1];

	const NINE_FOURTHS: f64 = 9.0 / 4.0;
	let mut current_power = NINE_FOURTHS;
	loop {
		current_power *= NINE_FOURTHS;
		let gap = ((current_power - 1.0) / (NINE_FOURTHS - 1.0)).ceil() as usize;

		if gap >= array_size {
			break gaps;
		}

		gaps.push(gap);
	}
}