pub struct Printer {}
impl Printer {
    pub fn new() -> Self {
        Printer {}
    }

    pub fn print(&self, line: &String) {
        println!("{line}");
    }
    pub fn print_group_separator(&self) {
        println!();
    }
    pub fn print_info(&self, header: &String, line: &String) {
        self.print(&format!("\x1b[93m{}\x1b[0m: {}", header, line));
    }
    pub fn print_warning(&self, header: &String, line: &String) {
        self.print(&format!("\x1b[38;5;208m{}\x1b[0m: {}", header, line));
    }
    pub fn print_error(&self, header: &String, line: &String) {
        self.print(&format!("\x1b[91m{}\x1b[0m: {}", header, line));
    }
    pub fn print_internal_error(&self, line: &str) {
        self.print(&format!("\x1b[91mBug\x1b[0m: {}", line));
    }
}
