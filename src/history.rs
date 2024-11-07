use crate::printer::Printer;
use crate::Cli;
use std::collections::HashMap;

pub struct History {
    last_fact: Option<String>,
    last_fact_count: u32,

    pub fact_occurrence: HashMap<String, u32>,
    pub fact_history: Vec<(String, u32)>,

    print_selected_facts: bool,
}

pub fn initialize_history(
    cli: &Cli,
) -> History {
    let all = cli.all || cli.print_all;

    History {
        last_fact: None,
        last_fact_count: 0,

        fact_occurrence: HashMap::new(),
        fact_history: Vec::new(),

        print_selected_facts: all || cli.print_selected_facts,
    }
}

impl History {
    pub fn register_selected_fact(&mut self, fact: String, printer: &mut dyn Printer) {
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
                self.fact_history
                    .push((last_fact_unwrap.clone(), self.last_fact_count));

                // output
                self.print_fact(printer, &last_fact_unwrap, self.last_fact_count, true);

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

        // output
        self.print_fact(printer, &fact, self.last_fact_count, false);
    }

    fn print_fact(&self, printer: &mut dyn Printer, fact: &str, count: u32, persistent: bool) {
        if !self.print_selected_facts {
            return;
        }

        if count > 1 {
            printer.print(format!("Selected ({count}x): {fact}"), persistent);
        } else {
            printer.print(format!("Selected: {fact}"), persistent);
        }
    }
}
