use crate::iteration_summary::IterationSummary;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

pub struct SaturationState {
    progress: Option<SaturationProgress>,
    query: Option<String>,
    hypothesis_fact_selected: Option<SelectedFact>,
    conclusion_fact_selected: Option<SelectedFact>,

    new_queue_entries: Vec<String>,

    iterations: Vec<Iteration>,
    pub hypothesis_selected_fact_history: Vec<(String, u32)>,
}

struct Iteration {
    progress: SaturationProgress,
    query: String,
    hypothesis_fact_selected: Option<SelectedFact>,
    conclusion_fact_selected: Option<SelectedFact>,

    new_queue_entries: Vec<String>,
}

#[derive(Copy, Clone)]
struct SaturationProgress {
    iteration: u32,
    with_conclusion_selected: u32,
    with_hypothesis_selected: u32,
    in_queue: u32,
}

#[derive(Clone, PartialEq, Eq)]
struct SelectedFact {
    fact: String,

    #[allow(dead_code)] // not yet used
    fact_number: Option<u32>,
}

impl Display for SaturationProgress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}c, {}h, {}q)", self.iteration, self.with_conclusion_selected, self.with_hypothesis_selected, self.in_queue,)
    }
}

impl SaturationState {
    pub fn new() -> Self {
        SaturationState {
            progress: None,
            query: None,
            conclusion_fact_selected: None,
            hypothesis_fact_selected: None,

            new_queue_entries: Vec::new(),

            iterations: Vec::new(),
            hypothesis_selected_fact_history: Vec::new(),
        }
    }

    pub fn set_query(&mut self, query: String) {
        self.query = Some(query);
    }

    pub fn set_queue_entry(&mut self, entry_number: u32, rule: String) {
        // ignore queue entries already existing in previous queue
        if let Some(last_iteration) = self.iterations.last() {
            if entry_number < last_iteration.progress.in_queue {
                return;
            }
        }

        self.new_queue_entries.push(rule);
    }

    pub fn set_hypothesis_fact_selected(&mut self, fact: String, fact_number: u32) {
        self.hypothesis_fact_selected = Some(SelectedFact { fact, fact_number: Some(fact_number) });
    }

    pub fn set_conclusion_fact_selected(&mut self, fact: String) {
        self.conclusion_fact_selected = Some(SelectedFact { fact, fact_number: None });
    }

    pub fn set_saturation_progress(&mut self, iteration: u32, with_conclusion_selected: u32, with_hypothesis_selected: u32, in_queue: u32) {
        self.progress = Some(SaturationProgress {
            iteration,
            with_conclusion_selected,
            with_hypothesis_selected,
            in_queue,
        });
    }

    pub fn complete_iteration(&mut self) -> Option<IterationSummary> {
        if self.progress.is_none() || self.query.is_none() {
            return None;
        }

        let progress = self.progress.unwrap();
        let query = self.query.clone().unwrap();

        let iteration = Iteration {
            progress,
            query,
            hypothesis_fact_selected: self.hypothesis_fact_selected.clone(),
            conclusion_fact_selected: self.conclusion_fact_selected.clone(),
            new_queue_entries: self.new_queue_entries.clone(),
        };

        // keep aggregated history of selected hypothesis (useful to detect loops)
        if let Some(previous_iteration) = self.iterations.last() {
            if previous_iteration.hypothesis_fact_selected.is_some() && previous_iteration.hypothesis_fact_selected == iteration.hypothesis_fact_selected {
                let last_index = self.hypothesis_selected_fact_history.len() - 1;
                let (fact, count) = self.hypothesis_selected_fact_history[last_index].clone();
                self.hypothesis_selected_fact_history[last_index] = (fact.clone(), count + 1);
            } else if let Some(hypothesis_fact_selected) = iteration.hypothesis_fact_selected.clone() {
                self.hypothesis_selected_fact_history.push((hypothesis_fact_selected.fact, 1))
            }
        }

        let selected_fact = Self::print_selected_fact(&iteration, &self.iterations.last());
        let summary = IterationSummary::new(selected_fact, iteration.query.clone(), iteration.new_queue_entries.clone(), format!("{}", iteration.progress));

        self.progress = None;
        self.query = None;
        self.hypothesis_fact_selected = None;
        self.conclusion_fact_selected = None;
        self.new_queue_entries = Vec::new();
        self.iterations.push(iteration);

        Some(summary)
    }

    fn print_selected_fact(iteration: &Iteration, previous_iteration: &Option<&Iteration>) -> String {
        let mut fact_source: &str = "";
        let mut fact: String = String::new();
        let mut same_as_before = false;

        if let Some(selected_fact) = iteration.hypothesis_fact_selected.clone() {
            fact_source = "hypothesis";
            fact = selected_fact.fact.clone();

            if let Some(previous_iteration) = previous_iteration {
                if let Some(previous_selected_fact) = previous_iteration.hypothesis_fact_selected.clone() {
                    same_as_before = previous_selected_fact.fact == selected_fact.fact
                }
            }
        } else if let Some(selected_fact) = iteration.conclusion_fact_selected.clone() {
            fact_source = "conclusion";
            fact = selected_fact.fact.clone();

            if let Some(previous_iteration) = previous_iteration {
                if let Some(previous_selected_fact) = previous_iteration.conclusion_fact_selected.clone() {
                    same_as_before = previous_selected_fact.fact == selected_fact.fact
                }
            }
        }

        let mut line = format!("{} {}", fact_source, fact);
        if same_as_before {
            line = format!("{line} (again)");
        }

        line
    }
}
