pub struct Printer {
    last_tag: Option<&'static str>,
}
pub fn initialize_printer() -> Printer {
    Printer { last_tag: None }
}
impl Printer {
    pub fn print_tag_aware(&mut self, line: String, overwrite_tag: Option<&'static str>) {
        let mut previous_line_ending = String::new();
        if let Some(last_tag) = self.last_tag.clone() {
            if let Some(overwrite_tag) = overwrite_tag.clone() {
                if last_tag != overwrite_tag {
                    previous_line_ending = "\n".to_string();
                }
            }
        }

        self.last_tag = overwrite_tag.clone();
        let line_ending = if overwrite_tag.is_none() { "\n" } else { "\r" };
        print!("{previous_line_ending}{line}{line_ending}");
    }

    pub fn print(&mut self, line: String) {
        self.print_tag_aware(line, None);
    }

    pub fn print_info(&mut self, header: String, line: String)
    {
        self.print(format!("\x1b[93m{}\x1b[0m: {}", header, line));
    }
    pub fn print_warning(&mut self, header: String, line: String)
    {
        self.print(format!("\x1b[38;5;208m{}\x1b[0m: {}", header, line));
    }
    pub fn print_error(&mut self, header: String, line: String) {

        self.print(format!("\x1b[91m{}\x1b[0m: {}", header, line));
    }
}