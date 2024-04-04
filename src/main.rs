use std::str::FromStr;

use argparse::{ArgumentParser, Print, Store, StoreTrue};
use gap_sequences::GapSequence;
use sort::{perform_rounds, ReportError, SortReport};

mod sort;
mod gap_sequences;

enum MaxDistanceOption {
    Length,
    Custom(f64),
}

impl FromStr for MaxDistanceOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(MaxDistanceOption::Length),
            _ => {
                let max_distance = s.parse::<f64>()
                .map_err(|_| "Invalid max distance provided.")?;

                Ok(MaxDistanceOption::Custom(max_distance))
            }
        }
    }
}

struct Options {
    seed: u64,
    rounds: usize,
    length: usize,
    gap_sequence: String,
    optimize: bool,
    optimize_moves: bool,
    quicksort: bool,
    max_distance: MaxDistanceOption,
    probability: f64,
}

fn main() {
    let mut options = Options { 
        seed: 0,
        rounds: 100,
        length: 100,
        gap_sequence: String::from(""),
        optimize: false,
        optimize_moves: false,
        quicksort: false,
        max_distance: MaxDistanceOption::Length,
        probability: 1.0,
    };
    
    let seed_help = format!(
        "Seed for random number generator. Default is {}.",
        options.seed,
    );
    let rounds_help = format!(
        "Number of rounds to run. Default is {}.",
        options.rounds
    );
    let length_help = format!(
        "Length of the array to be sorted. Default is {}.",
        options.length,
    );
    let probability_help = format!(
        "Probability of each element moving in a swap during shuffling. \
        Default is {}.",
        options.probability,
    );
    
    {
        let mut arg_parser = ArgumentParser::new();

        arg_parser.set_description(
            "Test the performance of Shellsort with different gap sequences."
        );

        arg_parser.add_option(
            &["-v", "--version"], 
            Print(env!("CARGO_PKG_VERSION").to_string()),
            "Show version",
        );

        arg_parser.refer(&mut options.seed)
        .add_option(
            &["-s", "--seed"], 
            Store,
            &seed_help,
        );

        arg_parser.refer(&mut options.rounds)
        .add_option(
            &["-r", "--rounds"], 
            Store,
            &rounds_help,
        );

        arg_parser.refer(&mut options.length)
        .add_option(
            &["-l", "--length"], 
            Store,
            &length_help,
        );

        arg_parser.refer(&mut options.gap_sequence)
        .add_option(
            &["-g", "--gap-sequence"],
            Store,
            "Specifies which gap sequence should be used. \
                Options are given by running this program with -g ls.",
        );

        arg_parser.refer(&mut options.optimize)
        .add_option(
            &["-o", "--optimize"],
            argparse::StoreTrue,
            "Adjust the gap sequence until it is optimal for the length.",
        );

        arg_parser.refer(&mut options.optimize_moves)
        .add_option(
            &["-m", "--optimize-moves"],
            argparse::StoreTrue,
            "If used with '-o', optimize for number of moves instead of comparisons.",
        );

        arg_parser.refer(&mut options.quicksort)
        .add_option(
            &["-q", "--quicksort"],
            StoreTrue,
            "Also run quicksort for comparison.",
        );

        arg_parser.refer(&mut options.max_distance)
        .add_option(
            &["-d", "--max-distance"],
            Store,
            "Maximum distance an element can move in a \
            swap during shuffling. Default is the length of the array.",
        );

        arg_parser.refer(&mut options.probability)
        .add_option(
            &["-p", "--probability"],
            Store,
            &probability_help,
        );
        
        arg_parser.parse_args_or_exit();
    }

    if options.gap_sequence.trim().to_lowercase() == "ls" {
        println!(include_str!("gap_sequences.txt"));
        return;
    }

    if !options.optimize {
        let gap_sequence = GapSequence::from_str(&options.gap_sequence)
        .expect("Invalid gap sequence provided.");

        let gaps: Vec<usize> = gap_sequence.to_vec(options.length);

        let max_distance = match options.max_distance {
            MaxDistanceOption::Length => options.length as f64,
            MaxDistanceOption::Custom(max_distance) => max_distance,
        }; 

        println!(
            "Sorting results on array of length {} for {} round(s) \
            and maximum swap distance of {} with {:.1}% probability of each swap.", 
            options.length,
            options.rounds,
            max_distance,
            options.probability * 100.0,
        );
        println!("Shell sort performed with gap sequence: {:?}", gaps);
        println!();

        let results = perform_rounds(
            options.length,
            options.seed,
            options.rounds,
            &gaps,
            options.quicksort,
            max_distance,
            options.probability,
        ).unwrap();

        let report = SortReport::try_from(results);

        match report {
            Ok(report) => {
                report.print();
            },
            Err(ReportError::EmptyResults) => {
                eprintln!("No results to report.")
            }
        }
    } else {
        //TODO: Here we should look for an optimal gap sequence for the given length.
    }
}