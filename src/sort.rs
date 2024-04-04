use std::cmp::{max, min};

use rand::{rngs::StdRng, Rng, SeedableRng};

use self::{
    quicksort::{
        quicksort, QuicksortReport, QuicksortResult
    }, 
    shellsort::{
        shellsort, ShellsortReport, ShellsortResult
    }
};

mod shellsort;
mod quicksort;

pub struct ShuffledAndSorted {
    shuffled: Vec<usize>,
    sorted: Vec<usize>,
    rng: StdRng
}

impl ShuffledAndSorted {
    pub fn new(length: usize, seed: u64) -> Self {
        let vec: Vec<usize> = (0..length).collect();
        Self {
            shuffled: vec.clone(),
            sorted: vec,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn shuffle(
        &mut self,
        max_distance: f64,
        probabilty: f64,
    ) -> Result<(), String> {
        if probabilty < 0.0 || probabilty > 1.0 {
            return Err("Probability must be between 0.0 and 1.0".to_string());
        }
        
        for i in 0..self.shuffled.len() {
            if !self.rng.gen_bool(probabilty) {
                continue;
            }

            let low = max(
                0,
                i as isize - max_distance as isize
            ) as usize;
            let high = min(
                self.shuffled.len() as isize - 1,
                i as isize + max_distance as isize
            ) as usize;

            let j = self.rng.gen_range(low..=high);
            self.shuffled.swap(i, j);
        }

        Ok(())
    }

    fn clone_shuffled(&mut self) {
        for i in 0..self.shuffled.len() {
            self.sorted[i] = self.shuffled[i];
        }
    }

    pub fn quicksort(&mut self) -> QuicksortResult {
        self.clone_shuffled();
        quicksort(&mut self.sorted)
    }

    pub fn shellsort(&mut self, gaps: &Vec<usize>) -> ShellsortResult {
        self.clone_shuffled();
        shellsort(&mut self.sorted, gaps)
    }
}

pub enum SortResults {
    ShellsortOnly(Vec<ShellsortResult>),
    ShellAndQuicksort {
        shellsort: Vec<ShellsortResult>,
        quicksort: Vec<QuicksortResult>,
    },
}

pub fn perform_rounds(
    length: usize,
    seed: u64,
    rounds: usize,
    gaps: &Vec<usize>,
    quicksort: bool,
    max_distance: f64,
    probability: f64,
) -> Result<SortResults, String> {
    let mut a: ShuffledAndSorted = ShuffledAndSorted::new(length, seed);

    let mut results: SortResults = if quicksort {
        SortResults::ShellAndQuicksort {
            shellsort: Vec::with_capacity(rounds),
            quicksort: Vec::with_capacity(rounds),
        }
    } else {
        SortResults::ShellsortOnly(Vec::with_capacity(rounds))
    };

    for _ in 0..rounds {
        a.shuffle(max_distance, probability)?;

        match results {
            SortResults::ShellsortOnly(ref mut shellsort_results) => {
                shellsort_results.push(a.shellsort(gaps));
            },
            SortResults::ShellAndQuicksort {
                ref mut shellsort,
                ref mut quicksort,
            } => {
                shellsort.push(a.shellsort(gaps));
                quicksort.push(a.quicksort());
            },
        }
    }

    Ok(results)
}

pub enum ReportError {
    EmptyResults,
}

pub enum SortReport {
    ShellsortOnly(ShellsortReport),
    ShellAndQuicksort{
        shellsort: ShellsortReport,
        quicksort: QuicksortReport,
    },
}

impl TryFrom<SortResults> for SortReport {
    type Error = ReportError;

    fn try_from(results: SortResults) -> Result<Self, ReportError> {
        match results {
            SortResults::ShellsortOnly(shellsort_results) => {
                let shellsort_report = ShellsortReport::try_from(shellsort_results)?;
                Ok(SortReport::ShellsortOnly(shellsort_report))
            }
            SortResults::ShellAndQuicksort { shellsort, quicksort } => {
                let shellsort_report = ShellsortReport::try_from(shellsort)?;
                let quicksort_report = QuicksortReport::try_from(quicksort)?;
                Ok(SortReport::ShellAndQuicksort {
                    shellsort: shellsort_report,
                    quicksort: quicksort_report,
                })
            }
        }
    }
}

impl SortReport {
    pub fn print(&self) {
        match self {
            SortReport::ShellsortOnly(ref report) => {
                report.print();
                println!();
            },
            SortReport::ShellAndQuicksort {
                ref shellsort,
                ref quicksort,
            } => {
                shellsort.print();
                println!();
                quicksort.print();
                println!()
            }
        }
    }
}