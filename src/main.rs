use std::collections::HashMap;
use std::io::{self};
use regex::Regex;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    all_selected_facts: bool,
}

fn main() {
    let cli = Cli::parse();

    let targets_map = compose_targets();
    let regex_map = compile_targets(targets_map);

    process_stdin(regex_map, cli);
}

fn compose_targets() -> HashMap<&'static str, Vec<&'static str>> {
    let mut templates_map: HashMap<&str, Vec<&str>> = HashMap::new();
    templates_map.insert("cyclic", vec![
        r"mess2\(.+,[0-9]{2,},.+\)", // detect 2-digit number in first channel
        r"mess2\(.+[0-9]{2,}\)", // detect 2-digit number in second channel
        r"table2\(.+[,\(][0-9]{2,}[,\(].+\)", // detect 2-digit number in table
    ]);

    templates_map
}

fn compile_targets(templates_map: HashMap<&'static str, Vec<&str>>) -> HashMap<&'static str, Vec<Regex>> {
    let regex_map: HashMap<&str, Vec<Regex>> = templates_map.iter()
        .map(|(k, v)| {
            let compiled_regexes = v.iter()
                .map(|pattern| Regex::new(pattern).unwrap())
                .collect();
            (*k, compiled_regexes)
        })
        .collect();
    regex_map
}

fn process_stdin(regex_map: HashMap<&str, Vec<Regex>>, cli: Cli) {
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
        let fact = captures_def.name("fact").unwrap().as_str();
        let fact_number = captures_def.name("fact_number").unwrap().as_str();

        // check for fact matches
        for group in regex_map.iter() {
            let group_header = group.0;
            let group_entries = group.1;

            for regex in group_entries.iter() {
                if !regex.is_match(fact) {
                    continue;
                }

                println!("Found {group_header} hypothesis pattern: {fact} (fact number: {fact_number}, pattern: {regex})");
            }
        }

        if cli.all_selected_facts {
            let cleaned_line = line.trim();
            println!("{cleaned_line}");
        }
    }
}