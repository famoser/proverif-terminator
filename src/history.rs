use std::collections::{HashMap};
use crate::Cli;
use crate::cycles::{find_cycles, Cycle};

pub struct History {
    last_fact: Option<String>,
    last_fact_count: u32,

    fact_occurrence: HashMap<String, u32>,
    fact_history: Vec<(String, u32)>,

    last_cycle: Option<Cycle>,
    last_cycle_end: usize,

    config: Config
}

struct Config {
    print_selected_facts: bool,
    print_cycles: bool,
}

pub fn initialize_history(cli: &Cli) -> History {
    let all = cli.all || cli.print_all;

    History {
        last_fact: None,
        last_fact_count: 0,

        fact_occurrence: HashMap::new(),
        fact_history: Vec::new(),

        last_cycle: None,
        last_cycle_end: 0,

        config: Config {
            print_selected_facts: all || cli.print_selected_facts,
            print_cycles: all || cli.print_loops,
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
        // early-out if cycle potentially still active (avoids spamming smaller cycles in big cycle)
        if self.last_cycle.is_some() {
            if self.fact_history.len() >= self.last_cycle_end {
                self.last_cycle = None;
            } else {
                return;
            }
        }

        if let Some(cycle) = find_cycles(&self.fact_history) {
            self.last_cycle = Some(cycle);
            self.last_cycle_end = self.fact_history.len() + cycle.size;

            if cycle.size * cycle.repeat > 1000 {
                println!("\x1b[91mCycle\x1b[0m: {:?}", cycle);
                return;
            }

            if cycle.size * cycle.repeat > 100 {
                println!("\x1b[38;5;208mCycle\x1b[0m: {:?}", cycle);
                return;
            }

            if cycle.size * cycle.repeat > 10 {
                println!("\x1b[93mCycle\x1b[0m: {:?}", cycle);
                return;
            }
        }
    }
}
