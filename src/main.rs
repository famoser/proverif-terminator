use std::collections::HashMap;
use std::io::{self};
use regex::Regex;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    all_hypothesis: bool,
}

fn main() {
    let args = Cli::parse();

    let targets_map = compose_targets(args);
    let regex_map = compile_targets(targets_map);

    process_stdin(regex_map);
}

fn compose_targets(cli: Cli) -> HashMap<&'static str, Vec<&'static str>> {
    let mut templates_map: HashMap<&str, Vec<&str>> = HashMap::new();
    templates_map.insert("cyclic", vec![
        r"Rule with hypothesis .+mess2\(.+,[0-9]{2,},.+,[0-9]{2,}\)",
    ]);

    if cli.all_hypothesis {
        templates_map.insert("hypothesis", vec![
            r"Rule with hypothesis .+",
        ]);
    }

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

fn process_stdin(regex_map: HashMap<&str, Vec<Regex>>) {
    let stdin = io::stdin();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        for group in regex_map.iter() {
            let group_header = group.0;
            let group_entries = group.1;

            for regex in group_entries.iter() {
                if regex.is_match(&line) {
                    let cleaned_line = line.trim();
                    println!("Found {group_header} pattern: {cleaned_line} (pattern: {regex})");
                }
            }
        }
    }
}