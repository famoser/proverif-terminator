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
    hypothesis_selected_fact: String,
    hypothesis_selected_fact_count: u32,
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
    }

    fn flush_hypothesis_iteration(
        &mut self,
        hypothesis_selected: &HypothesisSelected,
        printer: &mut Printer,
    ) {
        let mut last_fact_count = 0;
        if let Some(last_iteration_summary) = self.last_iteration_summary.clone() {
            // if different fact, fill history
            if last_iteration_summary.hypothesis_selected_fact != hypothesis_selected.fact {
                let fact = last_iteration_summary.hypothesis_selected_fact.clone();
                let fact_count = last_iteration_summary.hypothesis_selected_fact_count;
                self.hypothesis_selected_fact_history
                    .push((fact.clone(), fact_count));

                self.print_hypothesis_selected_fact(printer, &fact, fact_count, true);
            } else {
                // else keep count from before
                last_fact_count = last_iteration_summary.hypothesis_selected_fact_count
            }
        }

        let summary = IterationSummary {
            hypothesis_selected_fact: hypothesis_selected.fact.clone(),
            hypothesis_selected_fact_count: last_fact_count + 1,
        };
        self.print_hypothesis_selected_fact(
            printer,
            &summary.hypothesis_selected_fact,
            summary.hypothesis_selected_fact_count,
            false,
        );

        self.last_iteration_summary = Some(summary);
    }

    fn flush_conclusion_iteration(
        &mut self,
        conclusion_selected: &ConclusionSelected,
        printer: &mut Printer,
    ) {
        if let Some(last_iteration_summary) = self.last_iteration_summary.clone() {
            let fact = last_iteration_summary.hypothesis_selected_fact.clone();
            let fact_count = last_iteration_summary.hypothesis_selected_fact_count;
            self.hypothesis_selected_fact_history
                .push((fact, fact_count));

            self.last_iteration_summary = None
        }

        self.print_conclusion_selected_fact(printer, &conclusion_selected.fact)
    }

    fn print_hypothesis_selected_fact(
        &self,
        printer: &mut Printer,
        fact: &String,
        fact_count: u32,
        persistent: bool,
    ) {
        let mut line = String::new();
        if self.print_hypothesis_selected_fact {
            line = format!("Selected in hypothesis: {}", fact);
            if fact_count > 1 {
                line = format!("{line} ({}x)", fact_count);
            }
        }
        if self.print_saturation_progress && self.saturation_progress.is_some() {
            line = format!("{0}\t\t {line}", self.saturation_progress.unwrap());
        }

        if self.print_hypothesis_selected_fact || self.print_saturation_progress {
            let overwrite_tag = if persistent {
                None
            } else {
                Some("hypothesis_selected_fact")
            };
            printer.print_tag_aware(line.to_string(), overwrite_tag);
        }
    }

    fn print_conclusion_selected_fact(&self, printer: &mut Printer, fact: &String) {
        let mut line = String::new();
        if self.print_conclusion_selected_fact {
            line = format!("Selected in conclusion: {}", fact);
        }
        if self.print_saturation_progress && self.saturation_progress.is_some() {
            line = format!("{0}\t\t {line}", self.saturation_progress.unwrap());
        }

        if self.print_conclusion_selected_fact || self.print_saturation_progress {
            printer.print(line.to_string());
        }
    }
}
