use super::ReportError;


#[derive(Debug)]
pub struct QuicksortResult {
    pub comparisons: u32,
    pub swaps: u32,
    pub max_depth: u32,
}

pub fn quicksort<T: PartialOrd + Copy>(
    a: &mut Vec<T>,
) -> QuicksortResult {
    let mut result = QuicksortResult {
        comparisons: 0,
        swaps: 0,
        max_depth: 0,
    };

    quicksort_recursive(a, 0, a.len() - 1, &mut result, 0);

    result
}

fn quicksort_recursive<T: PartialOrd + Copy>(
    a: &mut Vec<T>,
    low: usize,
    high: usize,
    result: &mut QuicksortResult,
    depth: u32,
) {
    let depth = depth + 1;
    if depth > result.max_depth {
        result.max_depth = depth;
    }

    if low >= high {
        return;
    }

    let p = partition(a, low, high, result);

    if p > 0 {
        quicksort_recursive(a, low, p - 1, result, depth);
    }
    quicksort_recursive(a, p + 1, high, result, depth);
}

fn partition<T: PartialOrd + Copy>(
    a: &mut Vec<T>,
    low: usize,
    high: usize,
    result: &mut QuicksortResult
) -> usize {
    let pivot = a[high];
    let mut i = low;
    for j in low..high {
        result.comparisons += 1;
        if a[j] <= pivot {
            result.swaps += 1;
            a.swap(i, j);
            i += 1;
        }
    }

    result.swaps += 1;
    a.swap(i, high);

    i
}

pub struct QuicksortReport {
    pub average_comparisons: f64,
    pub average_swaps: f64,
    pub average_max_depth: f64,
    pub stddev_comparisons: f64,
    pub stddev_swaps: f64,
    pub stddev_max_depth: f64,
    pub fewest_comparisons: u32,
    pub fewest_swaps: u32,
    pub lowest_max_depth: u32,
    pub most_comparisons: u32,
    pub most_swaps: u32,
    pub highest_max_depth: u32,
}

impl QuicksortReport {
    pub fn print(&self) {
        println!("Quicksort Report:");
        println!("Average Comparisons: {:.2}", self.average_comparisons);
        println!("Std Dev Comparisons: {:.2}", self.stddev_comparisons);
        println!("Most Comparisons: {}", self.most_comparisons);
        println!("Fewest Comparisons: {}", self.fewest_comparisons);
        println!("Average Swaps: {:.2}", self.average_swaps);
        println!("Std Dev Swaps: {:.2}", self.stddev_swaps);
        println!("Most Swaps: {}", self.most_swaps);
        println!("Fewest Swaps: {}", self.fewest_swaps);
        println!("Average Max Depth: {:.2}", self.average_max_depth);
        println!("Std Dev Max Depth: {:.2}", self.stddev_max_depth);
        println!("Highest Max Depth: {}", self.highest_max_depth);
        println!("Lowest Max Depth: {}", self.lowest_max_depth);
    }

}

impl TryFrom<Vec<QuicksortResult>> for QuicksortReport {
    type Error = ReportError;

    fn try_from(results: Vec<QuicksortResult>) -> Result<Self, ReportError> {
        if results.is_empty() {
            return Err(ReportError::EmptyResults);
        }

        let mut report = Self {
            average_comparisons: 0.0,
            average_swaps: 0.0,
            average_max_depth: 0.0,
            stddev_comparisons: 0.0,
            stddev_swaps: 0.0,
            stddev_max_depth: 0.0,
            fewest_comparisons: u32::MAX,
            fewest_swaps: u32::MAX,
            lowest_max_depth: u32::MAX,
            most_comparisons: 0,
            most_swaps: 0,
            highest_max_depth: 0,
        };

        let mut sum_comparisons: u64 = 0;
        let mut sum_swaps: u64 = 0;
        let mut sum_max_depth: u64 = 0;

        for result in results.iter() {
            sum_comparisons += result.comparisons as u64;
            sum_swaps += result.swaps as u64;
            sum_max_depth += result.max_depth as u64;

            if result.comparisons < report.fewest_comparisons {
                report.fewest_comparisons = result.comparisons;
            }

            if result.swaps < report.fewest_swaps {
                report.fewest_swaps = result.swaps;
            }

            if result.max_depth < report.lowest_max_depth {
                report.lowest_max_depth = result.max_depth;
            }

            if result.comparisons > report.most_comparisons {
                report.most_comparisons = result.comparisons;
            }

            if result.swaps > report.most_swaps {
                report.most_swaps = result.swaps;
            }

            if result.max_depth > report.highest_max_depth {
                report.highest_max_depth = result.max_depth;
            }
        }

        let count = results.len() as f64;
        report.average_comparisons = sum_comparisons as f64 / count;
        report.average_swaps = sum_swaps as f64 / count;
        report.average_max_depth = sum_max_depth as f64 / count;

        let mut sum_squared_diff_comparisons: f64 = 0.0;
        let mut sum_squared_diff_swaps: f64 = 0.0;
        let mut sum_squared_diff_max_depth: f64 = 0.0;

        for result in results.iter() {
            let diff_comparisons = result.comparisons as f64 - report.average_comparisons;
            sum_squared_diff_comparisons += diff_comparisons * diff_comparisons;

            let diff_swaps = result.swaps as f64 - report.average_swaps;
            sum_squared_diff_swaps += diff_swaps * diff_swaps;

            let diff_max_depth = result.max_depth as f64 - report.average_max_depth;
            sum_squared_diff_max_depth += diff_max_depth * diff_max_depth;
        }

        report.stddev_comparisons = (sum_squared_diff_comparisons / count).sqrt();
        report.stddev_swaps = (sum_squared_diff_swaps / count).sqrt();
        report.stddev_max_depth = (sum_squared_diff_max_depth / count).sqrt();

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort_5() {
        let mut a = vec![3, 2, 1, 4, 5];
        quicksort(&mut a);

        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quicksort_10() {
        let mut a = vec![3, 2, 1, 4, 10, 5, 9, 8, 7, 6];
        quicksort(&mut a);

        assert_eq!(a, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_quicksort_reverse() {
        let mut a = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        quicksort(&mut a);

        assert_eq!(a, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_quicksort_duplicates() {
        let mut a = vec![3, 5, 5, 5, 2, 1, 4, 4, 5, 5];
        quicksort(&mut a);

        assert_eq!(a, vec![1, 2, 3, 4, 4, 5, 5, 5, 5, 5]);
    }
}