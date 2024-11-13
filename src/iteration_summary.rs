use crate::printer::Printer;
use crate::saturation_state::Iteration;
use crate::Cli;

pub struct IterationSummary {
    selected_fact: String,
    query: String,
    new_queue_entries: Vec<String>,
    ancestry: Option<Vec<Iteration>>,
    info: Vec<(String, String)>,
    warning: Vec<(String, String)>,
    error: Vec<(String, String)>,
    progress: String,
}

impl IterationSummary {
    pub fn new(selected_fact: String, query: String, new_queue_entries: Vec<String>, progress: String) -> Self {
        IterationSummary {
            selected_fact,
            query,
            new_queue_entries,
            ancestry: None,
            info: vec![],
            warning: vec![],
            error: vec![],
            progress,
        }
    }

    pub(crate) fn add_ancestry(&mut self, ancestry: Vec<Iteration>) {
        self.ancestry = Some(ancestry)
    }

    pub fn add_info(&mut self, header: String, line: String) {
        self.info.push((header, line));
    }
    pub fn add_warning(&mut self, header: String, line: String) {
        self.warning.push((header, line));
    }
    pub fn add_error(&mut self, header: String, line: String) {
        self.error.push((header, line));
    }

    pub fn print(&self, cli: &Cli, printer: &Printer) {
        let description = self.describe_iteration(cli);

        let total_output = description.len() + self.info.len() + self.warning.len() + self.error.len();
        if total_output == 0 {
            printer.print(&format!("{}\tSelected: {}", &self.progress, &self.selected_fact));
            return;
        }

        printer.print_group_separator();
        printer.print(&format!("Selected: {}", &self.selected_fact));
        for entry in description {
            printer.print(&entry);
        }
        for (header, line) in self.info.iter() {
            printer.print_info(header, line)
        }
        for (header, line) in self.warning.iter() {
            printer.print_warning(header, line)
        }
        for (header, line) in self.error.iter() {
            printer.print_error(header, line)
        }

        printer.print(&format!("Total: {}", self.progress));
        printer.print_group_separator();
    }

    fn describe_iteration(&self, cli: &Cli) -> Vec<String> {
        let all = cli.all || cli.print_all;

        let mut description = Vec::new();
        if let Some(ancestry) = self.ancestry.clone() {
            description.push("Ancestors:".to_string());
            for ancestor in ancestry.iter().rev() {
                let iteration_description = Self::describe_selected_iteration(&ancestor);
                description.push(format!("- {}", iteration_description));
            }
        }
        if all || cli.print_query {
            description.push(format!("Query: {}", &self.query));
        }
        if all || cli.print_new_queue_entries {
            for queue_entry in self.new_queue_entries.iter() {
                description.push(format!("New in queue: {}", queue_entry));
            }
        }

        description
    }

    fn describe_selected_iteration(iteration: &Iteration) -> String {
        let mut description: String = "".to_string();
        if let Some(hypothesis_fact_selected) = iteration.hypothesis_fact_selected.clone() {
            description = format!("hypothesis fact {} selected", hypothesis_fact_selected.fact_number.unwrap());
        };
        if iteration.conclusion_fact_selected.is_some() {
            description = "conclusion selected\t".to_string();
        };

        format!("{}\t{}\t{}", iteration.progress.iteration, description, iteration.query)
    }
}
