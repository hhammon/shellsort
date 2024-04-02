use std::vec;

use argparse::{ArgumentParser, Print};
use sort::{perform_rounds, ReportError, SortReport};

mod sort;

struct Options {
    seed: u64,
    rounds: usize,
    length: usize,
    gap_count: usize,
    custom_gaps: String,
    quicksort: bool,
}

fn main() {
    let mut options = Options { 
        seed: 0,
        rounds: 100,
        length: 100,
        gap_count: 8,
        custom_gaps: String::from(""),
        quicksort: false,
    };
    
    let seed_help = format!("Seed for random number generator. Default is {}.", options.seed);
    let rounds_help = format!("Number of rounds to run. Default is {}.", options.rounds);
    let length_help = format!("Length of the array to be sorted. Default is {}.", options.length);
    let gap_count_help = format!("Number of gaps to generate. Default is {}.", options.gap_count);
    
    {
        let mut arg_parser = ArgumentParser::new();

        arg_parser.set_description("Test the performance of Shellsort with different gap sequences.");

        arg_parser.add_option(
            &["-v", "--version"], 
            Print(env!("CARGO_PKG_VERSION").to_string()),
            "Show version",
        );

        arg_parser.refer(&mut options.seed)
        .add_option(
            &["-s", "--seed"], 
            argparse::Store,
            &seed_help,
        );

        arg_parser.refer(&mut options.rounds)
        .add_option(
            &["-r", "--rounds"], 
            argparse::Store,
            &rounds_help,
        );

        arg_parser.refer(&mut options.length)
        .add_option(
            &["-l", "--length"], 
            argparse::Store,
            &length_help,
        );

        arg_parser.refer(&mut options.gap_count)
        .add_option(
            &["-g", "--gap-count"], 
            argparse::Store,
            &gap_count_help,
        );

        arg_parser.refer(&mut options.custom_gaps)
        .add_option(
            &["-c", "--custom-gaps"],
            argparse::Store,
            "Use custom gaps for shell sort instead of run general test. Should be a comma separated list of positive integers. '1' is already prepended. The gap count option will be ignored.",
        );

        arg_parser.refer(&mut options.quicksort)
        .add_option(
            &["-q", "--quicksort"],
            argparse::StoreTrue,
            "Also run quicksort for comparison.",
        );
        
        arg_parser.parse_args_or_exit();
    }

    if !options.custom_gaps.is_empty() {
        let mut gaps: Vec<usize> = vec![1];
        gaps.extend(
            options.custom_gaps
            .split(",")
            .map(|s| s.trim().parse::<usize>().expect("Invalid gap provided."))
        );

        let results = perform_rounds(
            options.length,
            options.seed,
            options.rounds,
            &gaps,
            options.quicksort,
        );

        let report = SortReport::try_from(results);

        match report {
            Ok(report) => {
                println!(
                    "Sorting results on array of length {} for {} round(s).", 
                    options.length,
                    options.rounds,
                );
                println!("Shell sort performed with gap sequence: {:?}", gaps);
                println!();

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