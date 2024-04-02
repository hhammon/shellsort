use super::ReportError;


#[derive(Debug)]
pub struct ShellsortResult {
    pub comparisons: u32,
    pub moves: u32,
}

pub fn shellsort<T: PartialOrd + Copy>(
    a: &mut Vec<T>,
    gaps: &Vec<usize>,
) -> ShellsortResult {
    let mut result = ShellsortResult {
        comparisons: 0,
        moves: 0,
    };

    for gap in gaps.iter().rev() {
        for i in *gap..a.len() {
            let mut j = i;
            let temp = a[i];

            while j >= *gap {
                result.comparisons += 1;
                if a[j - gap] <= temp {
                    break;
                }

                result.moves += 1;
                a[j] = a[j - gap];

                j -= gap;
            }

            result.moves += 1;
            a[j] = temp;
        }
    }

    result
}

pub struct ShellsortReport {
    pub average_comparisons: f64,
    pub average_moves: f64,
    pub stddev_comparisons: f64,
    pub stddev_moves: f64,
    pub fewest_comparisons: u32,
    pub fewest_moves: u32,
    pub most_comparisons: u32,
    pub most_moves: u32,
}

impl ShellsortReport {
    pub fn print(&self) {
        println!("Shellsort Report:");
        println!("Average comparisons: {:.2}", self.average_comparisons);
        println!("Std Dev comparisons: {:.2}", self.stddev_comparisons);
        println!("Most comparisons: {}", self.most_comparisons);
        println!("Fewest comparisons: {}", self.fewest_comparisons);
        println!("Average moves: {:.2}", self.average_moves);
        println!("Std Dev moves: {:.2}", self.stddev_moves);
        println!("Most moves: {}", self.most_moves);
        println!("Fewest moves: {}", self.fewest_moves);
    }
}

impl TryFrom<Vec<ShellsortResult>> for ShellsortReport {
    type Error = ReportError;

    fn try_from(results: Vec<ShellsortResult>) -> Result<Self, ReportError> {
        if results.is_empty() {
            return Err(ReportError::EmptyResults);
        }

        let mut report = Self {
            average_comparisons: 0.0,
            average_moves: 0.0,
            stddev_comparisons: 0.0,
            stddev_moves: 0.0,
            fewest_comparisons: u32::MAX,
            fewest_moves: u32::MAX,
            most_comparisons: 0,
            most_moves: 0,
        };

        let mut sum_comparisons: u64 = 0;
        let mut sum_moves: u64 = 0;

        for result in results.iter() {
            sum_comparisons += result.comparisons as u64;
            sum_moves += result.moves as u64;

            if result.comparisons < report.fewest_comparisons {
                report.fewest_comparisons = result.comparisons;
            }

            if result.moves < report.fewest_moves {
                report.fewest_moves = result.moves;
            }

            if result.comparisons > report.most_comparisons {
                report.most_comparisons = result.comparisons;
            }

            if result.moves > report.most_moves {
                report.most_moves = result.moves;
            }
        }

        let count = results.len() as f64;
        report.average_comparisons = sum_comparisons as f64 / count;
        report.average_moves = sum_moves as f64 / count;

        let mut sum_squared_diff_comparisons: f64 = 0.0;
        let mut sum_squared_diff_moves: f64 = 0.0;

        for result in results.iter() {
            let diff_comparisons = result.comparisons as f64 - report.average_comparisons;
            sum_squared_diff_comparisons += diff_comparisons * diff_comparisons;

            let diff_moves = result.moves as f64 - report.average_moves;
            sum_squared_diff_moves += diff_moves * diff_moves;
        }

        report.stddev_comparisons = (sum_squared_diff_comparisons / count).sqrt();
        report.stddev_moves = (sum_squared_diff_moves / count).sqrt();

        Ok(report)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shellsort_5() {
        let mut a = vec![3, 2, 1, 4, 5];
        shellsort(&mut a, &vec![1, 4, 10, 23, 57]);

        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_shellsort_10() {
        let mut a = vec![3, 2, 1, 4, 10, 5, 9, 8, 7, 6];
        shellsort(&mut a, &vec![1, 4, 10, 23, 57]);

        assert_eq!(a, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_shellsort_reverse() {
        let mut a = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        shellsort(&mut a, &vec![1, 4, 10, 23, 57]);

        assert_eq!(a, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_shellsort_duplicates() {
        let mut a = vec![3, 5, 5, 5, 2, 1, 4, 4, 5, 5];
        shellsort(&mut a, &vec![1, 4, 10, 23, 57]);

        assert_eq!(a, vec![1, 2, 3, 4, 4, 5, 5, 5, 5, 5]);
    }
}