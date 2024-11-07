mod cycles;
mod fact_checker;
mod history;
mod printer;

use crate::cycles::{initialize_cycle_detector, CycleDetector};
use crate::fact_checker::{initialize_fact_checker, FactChecker};
use crate::history::{initialize_history, History};
use crate::printer::{initialize_queue_printer, QueuePrinter};
use clap::Parser;
use regex::Regex;
use std::io::{self};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    all: bool,

    #[arg(short, long)]
    print_all: bool,
    #[arg(long)]
    print_selected_facts: bool,
    #[arg(long)]
    print_cycles: bool,
    #[arg(long)]
    print_queue_state: bool,

    #[arg(short, long)]
    detect_all: bool,
    #[arg(long)]
    detect_high_counters: bool,
}

fn main() {
    let cli = Cli::parse();

    let fact_checker = initialize_fact_checker(&cli);
    let mut history = initialize_history(&cli);
    let mut cycle_detector = initialize_cycle_detector(&cli);
    let mut queue_printer = initialize_queue_printer(&cli);

    let stdin = io::stdin();
    let hypothesis_match =
        Regex::new(r"Rule with hypothesis fact (?<fact_number>[0-9]+) selected: (?<fact>.+)")
            .unwrap();
    let queue_match =
        Regex::new(r"(?<rules_inserted_count>\d+) rules inserted\. Base: (?<rules_base_count>\d+) rules \((?<rules_conclusion_selected_count>\d+) with conclusion selected\)\. Queue: (?<rules_queue_count>\d+) rules\.")
            .unwrap();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        if process_queue_fact(&queue_match, &mut line, &mut queue_printer) {
            continue;
        }
        if process_hypothesis_fact(&hypothesis_match, &mut line, &mut queue_printer, &mut history, &mut cycle_detector, &fact_checker) {
            continue;
        }
    }
}

fn process_queue_fact(
    queue_match: &Regex,
    line: &mut String,
    queue_printer: &mut QueuePrinter,
) -> bool {
    let captures = queue_match.captures(&line);
    if captures.is_none() {
        return false;
    }

    let captures_def = captures.unwrap();
    let rules_base_count: &str = captures_def.name("rules_base_count").unwrap().as_str();
    let rules_conclusion_selected_count: &str = captures_def
        .name("rules_conclusion_selected_count")
        .unwrap()
        .as_str();
    let rules_queue_count: &str = captures_def.name("rules_queue_count").unwrap().as_str();

    // stats
    let in_queue = rules_queue_count.parse::<u32>().unwrap_or(0);
    let with_conclusion_selected = rules_conclusion_selected_count.parse::<u32>().unwrap_or(0);
    let with_hypothesis_selected =
        rules_base_count.parse::<u32>().unwrap_or(0) - with_conclusion_selected;
    queue_printer.update_queue_state(in_queue, with_hypothesis_selected, with_conclusion_selected);

    true
}
fn process_hypothesis_fact<'a>(
    hypothesis_match: &Regex,
    line: &mut String,
    queue_printer: &mut QueuePrinter,
    history: &mut History,
    cycle_detector: &mut CycleDetector,
    fact_checker: &FactChecker,
) -> bool {
    let captures = hypothesis_match.captures(&line);
    if captures.is_none() {
        return false;
    }

    let captures_def = captures.unwrap();
    let fact: &str = captures_def.name("fact").unwrap().as_str();

    // stats
    history.register_selected_fact(fact.to_string(), queue_printer);
    fact_checker.check(fact);

    true
}
