use crate::printer::Printer;
use crate::Cli;
use std::fmt::{Display, Formatter};

pub struct SaturationState {
    saturation_progress: Option<SaturationProgress>,
    hypothesis_selected: Option<HypothesisSelected>,
    conclusion_selected: Option<ConclusionSelected>,

    pub hypothesis_selected_fact_history: Vec<(String, u32)>,
    last_iteration_summary: Option<IterationSummary>,

    print_saturation_progress: bool,
    print_hypothesis_selected_fact: bool,
    print_conclusion_selected_fact: bool,
}

#[derive(Copy, Clone)]
struct SaturationProgress {
    iteration: u32,
    with_conclusion_selected: u32,
    with_hypothesis_selected: u32,
    in_queue: u32,
}

#[derive(Clone)]
struct HypothesisSelected {
    fact: String,

    #[allow(dead_code)] // not yet used
    fact_number: u32,
}

#[derive(Clone)]
struct ConclusionSelected {
    fact: String,
}

#[derive(Clone)]
struct IterationSummary {
    hypothesis_selected_fact: Option<String>,
    conclusion_selected_fact: Option<String>,
}

pub fn initialize_saturation_state(cli: &Cli) -> SaturationState {
    let all = cli.all || cli.print_all;

    SaturationState {
        saturation_progress: None,
        hypothesis_selected: None,
        conclusion_selected: None,

        last_iteration_summary: None,
        hypothesis_selected_fact_history: Vec::new(),

        print_saturation_progress: all || cli.print_saturation_progress,
        print_hypothesis_selected_fact: all || cli.print_hypothesis_selected_fact,
        print_conclusion_selected_fact: all || cli.print_conclusion_selected_fact,
    }
}

impl Display for SaturationProgress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}c, {}h, {}q)",
            self.iteration,
            self.with_conclusion_selected,
            self.with_hypothesis_selected,
            self.in_queue,
        )
    }
}

impl SaturationState {
    pub fn set_hypothesis_selected(&mut self, fact: String, fact_number: u32) {
        self.hypothesis_selected = Some(HypothesisSelected { fact, fact_number });
    }

    pub fn set_conclusion_selected(&mut self, fact: String) {
        self.conclusion_selected = Some(ConclusionSelected { fact });
    }

    pub fn set_saturation_progress(
        &mut self,
        iteration: u32,
        with_conclusion_selected: u32,
        with_hypothesis_selected: u32,
        in_queue: u32,
    ) {
        self.saturation_progress = Some(SaturationProgress {
            iteration,
            with_conclusion_selected,
            with_hypothesis_selected,
            in_queue,
        });
    }

    pub fn flush_iteration(&mut self, printer: &mut Printer) {
        if let Some(hypothesis_selected) = self.hypothesis_selected.clone() {
            self.flush_hypothesis_iteration(&hypothesis_selected, printer);
        } else if let Some(conclusion_selected) = self.conclusion_selected.clone() {
            self.flush_conclusion_iteration(&conclusion_selected, printer);
        }

        self.hypothesis_selected = None;
        self.conclusion_selected = None;
        self.saturation_progress = None;
    }

    fn flush_hypothesis_iteration(
        &mut self,
        hypothesis_selected: &HypothesisSelected,
        printer: &mut Printer,
    ) {
        let mut same_as_before = false;
        if let Some(last_iteration_summary) = self.last_iteration_summary.clone() {
            // if different fact, fill history
            if let Some(hypothesis_selected_fact) = last_iteration_summary.hypothesis_selected_fact {
                same_as_before = hypothesis_selected_fact == hypothesis_selected.fact
            }
        }

        self.print_iteration(printer, "hypothesis", self.print_hypothesis_selected_fact, &hypothesis_selected.fact, same_as_before);

        self.last_iteration_summary = Some(IterationSummary {
            hypothesis_selected_fact: Some(hypothesis_selected.fact.clone()),
            conclusion_selected_fact: None,
        });
    }

    fn flush_conclusion_iteration(
        &mut self,
        conclusion_selected: &ConclusionSelected,
        printer: &mut Printer,
    ) {
        let mut same_as_before = false;
        if let Some(last_iteration_summary) = self.last_iteration_summary.clone() {
            // if different fact, fill history
            if let Some(conclusion_selected_fact) = last_iteration_summary.conclusion_selected_fact {
                same_as_before = conclusion_selected_fact == conclusion_selected.fact
            }
        }

        self.print_iteration(printer, "conclusion", self.print_conclusion_selected_fact, &conclusion_selected.fact, same_as_before);

        self.last_iteration_summary = Some(IterationSummary {
            hypothesis_selected_fact: Some(conclusion_selected.fact.clone()),
            conclusion_selected_fact: None,
        });
    }


    fn print_iteration(
        &self,
        printer: &mut Printer,
        location: &str,
        print_location: bool,
        fact: &String,
        same_as_before: bool
    ) {
        let mut line = String::new();
        if print_location {
            line = format!("Selected in {}: {}", location, fact);
            if same_as_before {
                line = format!("{line} (again)");
            }
        }
        if self.print_saturation_progress && self.saturation_progress.is_some() {
            line = format!("{0}\t{line}", self.saturation_progress.unwrap());
        }

        if print_location || self.print_saturation_progress {
            printer.print(line.to_string());
        }
    }
}
