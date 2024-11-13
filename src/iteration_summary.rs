use crate::printer::Printer;
use crate::Cli;

pub struct IterationSummary {
    selected_fact: String,
    query: String,
    new_queue_entries: Vec<String>,
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
            info: vec![],
            warning: vec![],
            error: vec![],
            progress,
        }
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

    pub fn print(&mut self, cli: &Cli, printer: &Printer) {
        let all = cli.all || cli.print_all;

        printer.print(&format!("Selected: {}", &self.selected_fact));
        if all || cli.print_query {
            printer.print(&format!("Query: {}", &self.query));
        }
        if all || cli.print_new_queue_entries {
            for queue_entry in self.new_queue_entries.iter() {
                printer.print(&format!("New in queue: {}", queue_entry));
            }
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
}
