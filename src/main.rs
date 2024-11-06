mod history;
mod cycles;
mod fact_checker;

use std::io::{self};
use regex::Regex;
use clap::Parser;
use crate::fact_checker::initialize_fact_checker;
use crate::history::{initialize_history};

#[derive(Parser)]
struct Cli {
    #[arg(short,long)]
    all: bool,

    #[arg(short,long)]
    print_all: bool,
    #[arg(long)]
    print_selected_facts: bool,
    #[arg(long)]
    print_loops: bool,


    #[arg(short,long)]
    detect_all: bool,
    #[arg(long)]
    detect_high_counters: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut history = initialize_history(&cli);
    let fact_checker = initialize_fact_checker(&cli);

    let stdin = io::stdin();
    let hypothesis_match = Regex::new(r"Rule with hypothesis fact (?<fact_number>[0-9]+) selected: (?<fact>.+)").unwrap();

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        let captures = hypothesis_match.captures(&line);
        if captures.is_none() {
            continue;
        }

        let captures_def = captures.unwrap();
        let fact: &str = captures_def.name("fact").unwrap().as_str();

        // stats
        history.register_selected_fact(fact.to_string());
        fact_checker.check(fact);
    }
}
