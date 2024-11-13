use crate::printer::Printer;

pub struct IterationSummary {
    title: String,
    info: Vec<(String, String)>,
    warning: Vec<(String, String)>,
    error: Vec<(String, String)>,
    summary: String,
}

impl IterationSummary {
    pub fn new(title: String, summary: String) -> Self {
        IterationSummary {
            title,
            info: vec![],
            warning: vec![],
            error: vec![],
            summary,
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

    pub fn print(&mut self, printer: &Printer) {
        printer.print(&self.title);
        for (header, line) in self.info.iter() {
            printer.print_info(header, line)
        }
        for (header, line) in self.warning.iter() {
            printer.print_warning(header, line)
        }
        for (header, line) in self.error.iter() {
            printer.print_error(header, line)
        }

        printer.print(&format!("total: {}", self.summary));
        printer.print_group_separator();
    }
}
