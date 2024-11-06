use std::collections::{HashMap};
use crate::Cli;
use crate::cycles::{find_circles};

pub struct History {
    last_fact: Option<String>,
    last_fact_count: u32,

    fact_occurrence: HashMap<String, u32>,
    fact_history: Vec<(String, u32)>,

    config: Config
}

struct Config {
    print_selected_facts: bool,
    print_cycles: bool,
}

pub fn initialize_history(cli: &Cli) -> History {
    History {
        last_fact: None,
        last_fact_count: 0,

        fact_occurrence: HashMap::new(),
        fact_history: Vec::new(),

        config: Config {
            print_selected_facts: cli.print_all || cli.print_selected_facts,
            print_cycles: cli.print_all || cli.print_loops,
        }
    }
}

impl History {
    pub fn register_selected_fact(&mut self, fact: String) {
        *self.fact_occurrence.entry(fact.clone()).or_insert(0) += 1;

        // first invocation
        if self.last_fact.is_none() {
            self.last_fact = Some(fact.clone());
            self.last_fact_count = 1;

            return;
        }

        if let Some(last_fact_unwrap) = self.last_fact.clone() {
            // if different fact, fill history
            if last_fact_unwrap != fact {
                self.fact_history.push((last_fact_unwrap.clone(), self.last_fact_count));

                if self.config.print_selected_facts { Self::print_fact(&last_fact_unwrap, self.last_fact_count, false); }
                if self.config.print_cycles { self.detect_and_print_cycles(); }

                // reset
                self.last_fact_count = 0;
                self.last_fact = Some(fact.clone());
            }
        } else {
            // first invocation
            self.last_fact_count = 0;
            self.last_fact = Some(fact.clone());
        }

        self.last_fact_count += 1;

        if self.config.print_selected_facts { Self::print_fact(&fact, self.last_fact_count, true); }
    }

    fn print_fact(fact: &str, count: u32, intermediate: bool) {
        let line_ending = if intermediate { "\r" } else { "\n" };
        if count > 1 {
            print!("Choosing ({count}x): {fact} {line_ending}");
        } else {
            print!("Choosing: {fact}{line_ending}");
        }
    }

    fn detect_and_print_cycles(&mut self) {
        let circle_size = find_circles(&self.fact_history);
        if let Some(circle) = circle_size {
            print!("\\e[33mCircle[0m: {:?}", circle);
        }
    }
}
