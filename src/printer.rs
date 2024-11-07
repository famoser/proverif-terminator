use crate::Cli;
use std::fmt::{Display, Formatter};

pub struct QueuePrinter {
    queue_state: Option<QueueState>,
    last_transient_print: String,
    print_queue_state: bool,
}

#[derive(Copy, Clone)]
struct QueueState {
    with_hypothesis_selected: u32,
    with_conclusion_selected: u32,
    in_queue: u32,
}

impl Display for QueueState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "q: {}, h: {}, c: {}",
            self.in_queue, self.with_hypothesis_selected, self.with_conclusion_selected,
        )
    }
}

pub fn initialize_queue_printer(cli: &Cli) -> QueuePrinter {
    let all = cli.all || cli.print_all;

    QueuePrinter {
        queue_state: None,
        last_transient_print: String::from(""),
        print_queue_state: all || cli.print_queue_state,
    }
}

pub trait Printer {
    fn print(&mut self, line: String, persistent: bool);
    fn print_transient(&mut self, line: String);
    fn print_persistent(&mut self, line: String);
}

impl Printer for QueuePrinter {
    fn print(&mut self, line: String, persistent: bool) {
        if persistent {
            self.print_persistent(line);
        } else {
            self.print_transient(line);
        }
    }

    fn print_transient(&mut self, line: String) {
        self.last_transient_print = line.clone();
        self.print_internal(&self.last_transient_print, false);
    }

    fn print_persistent(&mut self, line: String) {
        self.last_transient_print = String::from("");
        self.print_internal(&line, true);
    }
}

impl QueuePrinter {
    pub fn update_queue_state(
        &mut self,
        in_queue: u32,
        with_hypothesis_selected: u32,
        with_conclusion_selected: u32,
    ) {
        self.queue_state = Some(QueueState {
            in_queue,
            with_hypothesis_selected,
            with_conclusion_selected,
        });

        self.print_internal(&self.last_transient_print, false);
    }

    fn print_internal(&self, line: &String, persistent: bool) {
        let line_ending = if persistent { "\n" } else { "\r" };
        if self.print_queue_state && self.queue_state.is_some() {
            print!("({0}): {line}{line_ending}", self.queue_state.unwrap());
        } else {
            print!("{line}{line_ending}");
        }
    }
}
