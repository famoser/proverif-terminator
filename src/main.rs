mod cycles;
mod fact_checker;
mod iteration_summary;
mod printer;
mod rule_explainer;
mod saturation_state;

use crate::cycles::CycleDetector;
use crate::fact_checker::FactChecker;
use crate::printer::Printer;
use crate::rule_explainer::QueryExplainer;
use crate::saturation_state::SaturationState;
use clap::Parser;
use regex::Regex;
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

    #[arg(short, long)]
    print_all: bool,
    #[arg(long)]
    print_query: bool,
    #[arg(long)]
    print_new_queue_entries: bool,

    #[arg(long)]
    explain_query: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    let mut saturation_state = SaturationState::new();

    let fact_checker = FactChecker::new(&cli);
    let mut cycle_detector = CycleDetector::new();

    let printer = Printer::new();

    let stdin = io::stdin();

    let hypothesis_match = Regex::new(r"Rule with hypothesis fact (?<fact_number>[0-9]+) selected: (?<fact>.+)").unwrap();
    let conclusion_match = Regex::new(r"Rule with conclusion selected:").unwrap();
    let conclusion_fact_match = Regex::new(r".+ -> (?<fact>.+)").unwrap();
    let progress_match = Regex::new(r"(?<rules_inserted_count>\d+) rules inserted\. Base: (?<rules_base_count>\d+) rules \((?<rules_conclusion_selected_count>\d+) with conclusion selected\)\. Queue: (?<rules_queue_count>\d+) rules\.").unwrap();

    let conclusion_start_match = Regex::new(r"\*\*\* Rules with the conclusion selected").unwrap();
    let hypothesis_start_match = Regex::new(r"\*\*\* Rules with an hypothesis selected").unwrap();
    let queue_start_match = Regex::new(r"\*\*\* Rules in queue").unwrap();
    let rule_match = Regex::new(r"(?<rule_number>[0-9]+) -- (?<rule>.+)").unwrap();
    let mut rule_context = RuleContext::Unknown;

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        if let Some(rule_capture) = rule_match.captures(&line) {
            match rule_context {
                RuleContext::Queue => {}
                _ => {
                    continue;
                }
            }

            let rule = rule_capture.name("rule").unwrap().as_str();
            let rule_number = rule_capture.name("rule_number").unwrap().as_str();
            let rule_number = rule_number.parse::<usize>().unwrap_or(0);

            saturation_state.set_queue_entry(rule_number, rule.to_string());

            continue;
        }

        if conclusion_start_match.captures(&line).is_some() {
            rule_context = RuleContext::Conclusion;
        }

        if hypothesis_start_match.captures(&line).is_some() {
            rule_context = RuleContext::Hypothesis;
        }

        if queue_start_match.captures(&line).is_some() {
            rule_context = RuleContext::Queue;
        }

        if let Some(progress_capture) = progress_match.captures(&line) {
            let rules_inserted_count = progress_capture.name("rules_inserted_count").unwrap().as_str();
            let rules_base_count = progress_capture.name("rules_base_count").unwrap().as_str();
            let rules_conclusion_selected_count: &str = progress_capture.name("rules_conclusion_selected_count").unwrap().as_str();
            let rules_queue_count: &str = progress_capture.name("rules_queue_count").unwrap().as_str();

            let iteration = rules_inserted_count.parse::<usize>().unwrap_or(0);
            let in_queue = rules_queue_count.parse::<usize>().unwrap_or(0);
            let with_conclusion_selected = rules_conclusion_selected_count.parse::<usize>().unwrap_or(0);
            let with_hypothesis_selected = rules_base_count.parse::<usize>().unwrap_or(0) - with_conclusion_selected;
            saturation_state.set_saturation_progress(iteration, with_conclusion_selected, with_hypothesis_selected, in_queue);
            continue;
        }

        if let Some(hypothesis_capture) = hypothesis_match.captures(&line) {
            rule_context = RuleContext::Queue;
            flush_iteration(&cli, &mut saturation_state, &fact_checker, &mut cycle_detector, &printer);

            let mut query = String::new();
            stdin.read_line(&mut query).unwrap();
            saturation_state.set_query(query.clone().trim().to_string());

            let fact = hypothesis_capture.name("fact").unwrap().as_str();
            let fact_number = hypothesis_capture.name("fact_number").unwrap().as_str();

            let fact_number = fact_number.parse::<usize>().unwrap_or(0);
            saturation_state.set_hypothesis_fact_selected(fact.to_string(), fact_number);
            continue;
        }

        if conclusion_match.captures(&line).is_some() {
            rule_context = RuleContext::Queue;
            flush_iteration(&cli, &mut saturation_state, &fact_checker, &mut cycle_detector, &printer);

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

enum RuleContext {
    Unknown,
    Conclusion,
    Hypothesis,
    Queue,
}

fn flush_iteration(cli: &Cli, saturation_state: &mut SaturationState, fact_checker: &FactChecker, cycle_detector: &mut CycleDetector, printer: &Printer) {
    let iteration_summary = saturation_state.complete_iteration();
    if iteration_summary.is_none() {
        return;
    }

    let mut iteration_summary = iteration_summary.unwrap();

    if cli.detect_all || cli.detect_cycles {
        cycle_detector.check_cycles(&saturation_state.hypothesis_selected_fact_history, &mut iteration_summary);
    }

    if let Some(history) = saturation_state.hypothesis_selected_fact_history.last() {
        fact_checker.check(&history.0, &mut iteration_summary)
    }

    if let Some(explain_query) = cli.explain_query {
        if saturation_state.iterations.len() == explain_query {
            let explainer = QueryExplainer::new();
            let ancestry = explainer.get_ancestry(&saturation_state.iterations);
            iteration_summary.add_ancestry(ancestry);
        }
        cycle_detector.check_cycles(&saturation_state.hypothesis_selected_fact_history, &mut iteration_summary);
    }

    // print
    iteration_summary.print(cli, printer)
}
