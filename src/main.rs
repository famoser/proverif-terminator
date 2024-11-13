mod cycles;
mod fact_checker;
mod iteration_summary;
mod printer;
mod saturation_state;

use crate::cycles::CycleDetector;
use crate::fact_checker::FactChecker;
use crate::printer::Printer;
use crate::saturation_state::SaturationState;
use clap::Parser;
use regex::{Captures, Regex};
use std::io::{self};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    all: bool,

    #[arg(short, long)]
    detect_all: bool,
    #[arg(long)]
    detect_high_counters: bool,
    #[arg(long)]
    detect_cycles: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut saturation_state = SaturationState::new();

    let fact_checker = FactChecker::new(&cli);
    let mut cycle_detector = CycleDetector::new();

    let mut printer = Printer::new();

    let stdin = io::stdin();
    let hypothesis_match =
        Regex::new(r"Rule with hypothesis fact (?<fact_number>[0-9]+) selected: (?<fact>.+)")
            .unwrap();
    let conclusion_match = Regex::new(r"Rule with conclusion selected:").unwrap();
    let conclusion_fact_match = Regex::new(r".+ -> (?<fact>.+)").unwrap();
    let queue_match =
        Regex::new(r"(?<rules_inserted_count>\d+) rules inserted\. Base: (?<rules_base_count>\d+) rules \((?<rules_conclusion_selected_count>\d+) with conclusion selected\)\. Queue: (?<rules_queue_count>\d+) rules\.")
            .unwrap();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        let queue_capture = queue_match.captures(&line);
        if let Some(queue_capture) = queue_capture {
            process_queue_status(&queue_capture, &mut saturation_state);
            continue;
        }

        let hypothesis_capture = hypothesis_match.captures(&line);
        if let Some(hypothesis_capture) = hypothesis_capture {
            flush_iteration(
                &cli,
                &mut saturation_state,
                &fact_checker,
                &mut cycle_detector,
                &mut printer,
            );

            let mut query = String::new();
            stdin.read_line(&mut query).unwrap();
            saturation_state.set_query(query.clone().trim().to_string());

            let fact = hypothesis_capture.name("fact").unwrap().as_str();
            let fact_number = hypothesis_capture.name("fact_number").unwrap().as_str();

            let fact_number = fact_number.parse::<u32>().unwrap_or(0);
            saturation_state.set_hypothesis_fact_selected(fact.to_string(), fact_number);
            continue;
        }

        let conclusion_capture = conclusion_match.captures(&line);
        if conclusion_capture.is_some() {
            flush_iteration(
                &cli,
                &mut saturation_state,
                &fact_checker,
                &mut cycle_detector,
                &mut printer,
            );

            let mut query = String::new();
            stdin.read_line(&mut query).unwrap();
            saturation_state.set_query(query.clone().trim().to_string());

            let conclusion_fact_capture = conclusion_fact_match.captures(&query);
            if let Some(conclusion_fact_capture) = conclusion_fact_capture {
                let fact = conclusion_fact_capture.name("fact").unwrap().as_str();

                saturation_state.set_conclusion_fact_selected(fact.to_string());
            } else {
                saturation_state.set_conclusion_fact_selected(query.trim().to_string());
            }
            continue;
        }
    }
}

fn flush_iteration(
    cli: &Cli,
    saturation_state: &mut SaturationState,
    fact_checker: &FactChecker,
    cycle_detector: &mut CycleDetector,
    printer: &Printer,
) {
    saturation_state.complete_iteration(printer);
    if let Some(mut iteration_printer) = saturation_state.create_last_iteration_printer() {
        if cli.detect_all || cli.detect_cycles {
            cycle_detector.check_cycles(
                &saturation_state.hypothesis_selected_fact_history,
                &mut iteration_printer,
            );
        }

        if let Some(history) = saturation_state.hypothesis_selected_fact_history.last() {
            fact_checker.check(&history.0, &mut iteration_printer)
        }

        // print
        iteration_printer.print(printer)
    }
}

fn process_queue_status(captures: &Captures, saturation_state: &mut SaturationState) {
    let rules_inserted_count = captures.name("rules_inserted_count").unwrap().as_str();
    let rules_base_count = captures.name("rules_base_count").unwrap().as_str();
    let rules_conclusion_selected_count: &str = captures
        .name("rules_conclusion_selected_count")
        .unwrap()
        .as_str();
    let rules_queue_count: &str = captures.name("rules_queue_count").unwrap().as_str();

    let iteration = rules_inserted_count.parse::<u32>().unwrap_or(0);
    let in_queue = rules_queue_count.parse::<u32>().unwrap_or(0);
    let with_conclusion_selected = rules_conclusion_selected_count.parse::<u32>().unwrap_or(0);
    let with_hypothesis_selected =
        rules_base_count.parse::<u32>().unwrap_or(0) - with_conclusion_selected;
    saturation_state.set_saturation_progress(
        iteration,
        with_conclusion_selected,
        with_hypothesis_selected,
        in_queue,
    );
}
