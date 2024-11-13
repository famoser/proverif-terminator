use crate::iteration_summary::IterationSummary;
use crate::printer::Printer;
use std::fmt::{Display, Formatter};

pub struct SaturationState {
    progress: Option<SaturationProgress>,
    query: Option<String>,
    hypothesis_fact_selected: Option<SelectedFact>,
    conclusion_fact_selected: Option<SelectedFact>,

    queue_entries: Vec<String>,

    iterations: Vec<Iteration>,
    pub hypothesis_selected_fact_history: Vec<(String, u32)>,
}

struct Iteration {
    progress: SaturationProgress,
    #[allow(dead_code)] // not yet used
    query: String,
    hypothesis_fact_selected: Option<SelectedFact>,
    conclusion_fact_selected: Option<SelectedFact>,

    #[allow(dead_code)] // not yet used
    added_to_queue: Vec<String>,
}

#[derive(Copy, Clone)]
struct SaturationProgress {
    iteration: u32,
    with_conclusion_selected: u32,
    with_hypothesis_selected: u32,
    in_queue: u32,
}

#[derive(Clone)]
struct SelectedFact {
    fact: String,

    #[allow(dead_code)] // not yet used
    fact_number: Option<u32>,
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
    pub fn new() -> Self {
        SaturationState {
            progress: None,
            query: None,
            conclusion_fact_selected: None,
            hypothesis_fact_selected: None,

            queue_entries: Vec::new(),

            iterations: Vec::new(),
            hypothesis_selected_fact_history: Vec::new(),
        }
    }

    pub fn set_query(&mut self, query: String) {
        self.query = Some(query);
    }

    pub fn set_hypothesis_fact_selected(&mut self, fact: String, fact_number: u32) {
        self.hypothesis_fact_selected = Some(SelectedFact {
            fact,
            fact_number: Some(fact_number),
        });
    }

    pub fn set_conclusion_fact_selected(&mut self, fact: String) {
        self.conclusion_fact_selected = Some(SelectedFact {
            fact,
            fact_number: None,
        });
    }

    pub fn set_saturation_progress(
        &mut self,
        iteration: u32,
        with_conclusion_selected: u32,
        with_hypothesis_selected: u32,
        in_queue: u32,
    ) {
        self.progress = Some(SaturationProgress {
            iteration,
            with_conclusion_selected,
            with_hypothesis_selected,
            in_queue,
        });
    }

    pub fn complete_iteration(&mut self, printer: &Printer) {
        if let (Some(progress), Some(query)) = (self.progress, self.query.clone()) {
            let iteration = Iteration {
                progress,
                query,
                hypothesis_fact_selected: self.hypothesis_fact_selected.clone(),
                conclusion_fact_selected: self.conclusion_fact_selected.clone(),
                added_to_queue: Vec::new(),
            };

            self.iterations.push(iteration)

            // todo: check whether to extend hypothesis_selected_fact_history
        } else {
            printer.print_internal_error("Cannot create iteration")
        }

        self.progress = None;
        self.query = None;
        self.hypothesis_fact_selected = None;
        self.conclusion_fact_selected = None;
        self.queue_entries = Vec::new()
    }

    pub fn create_last_iteration_printer(&mut self) -> Option<IterationSummary> {
        if let Some(last_iteration) = self.iterations.last() {
            let previous_iteration = self.iterations.get(self.iterations.len() - 2);

            let title = Self::print_selected_fact(&last_iteration, &previous_iteration);
            let summary = IterationSummary::new(title, format!("{}", last_iteration.progress));

            Some(summary)
        } else {
            None
        }
    }

    fn print_selected_fact(
        iteration: &Iteration,
        previous_iteration: &Option<&Iteration>,
    ) -> String {
        let mut fact_source: &str = "";
        let mut fact: String = String::new();
        let mut same_as_before = false;

        if let Some(selected_fact) = iteration.hypothesis_fact_selected.clone() {
            fact_source = "hypothesis";
            fact = selected_fact.fact.clone();

            if let Some(previous_iteration) = previous_iteration {
                if let Some(previous_selected_fact) =
                    previous_iteration.hypothesis_fact_selected.clone()
                {
                    same_as_before = previous_selected_fact.fact == selected_fact.fact
                }
            }
        } else if let Some(selected_fact) = iteration.conclusion_fact_selected.clone() {
            fact_source = "conclusion";
            fact = selected_fact.fact.clone();

            if let Some(previous_iteration) = previous_iteration {
                if let Some(previous_selected_fact) =
                    previous_iteration.conclusion_fact_selected.clone()
                {
                    same_as_before = previous_selected_fact.fact == selected_fact.fact
                }
            }
        }

        let mut line = format!("{} fact selected: {}", fact_source, fact);
        if same_as_before {
            line = format!("{line} (again)");
        }

        line
    }
}
