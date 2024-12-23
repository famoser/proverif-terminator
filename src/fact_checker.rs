use crate::iteration_summary::IterationSummary;
use crate::Cli;
use regex::Regex;
use std::collections::HashMap;

pub struct FactChecker {
    regex_map: HashMap<&'static str, Vec<Regex>>,
}

impl FactChecker {
    pub fn new(cli: &Cli) -> Self {
        let templates_map = compose_targets(cli);
        let regex_map = compile_targets(templates_map);

        FactChecker { regex_map }
    }

    pub fn check(&self, fact: &str, printer: &mut IterationSummary) {
        for group in self.regex_map.iter() {
            let group_header = group.0;
            let group_entries = group.1;

            for regex in group_entries.iter() {
                if !regex.is_match(fact) {
                    continue;
                }

                printer.add_warning(format!("{} pattern", group_header), regex.to_string());
            }
        }
    }
}

fn compose_targets(cli: &Cli) -> HashMap<&'static str, Vec<&'static str>> {
    let all = cli.all || cli.detect_all;

    let mut templates_map: HashMap<&str, Vec<&str>> = HashMap::new();
    if all || cli.detect_high_counters {
        templates_map.insert(
            "HighCounter",
            vec![
                r"mess2\(.+,[0-9]{2,},.+\)",          // detect 2-digit number in first channel
                r"mess2\(.+,[0-9]{2,}\)",             // detect 2-digit number in second channel
                r"table2\(.+[,\(][0-9]{2,}[,\(].+\)", // detect 2-digit number in table
            ],
        );
    }

    templates_map
}

fn compile_targets(templates_map: HashMap<&'static str, Vec<&str>>) -> HashMap<&'static str, Vec<Regex>> {
    let regex_map: HashMap<&'static str, Vec<Regex>> = templates_map
        .iter()
        .map(|(k, v)| {
            let compiled_regexes = v.iter().map(|pattern| Regex::new(pattern).unwrap()).collect();
            (*k, compiled_regexes)
        })
        .collect();

    regex_map
}
