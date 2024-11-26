use crate::iteration_summary::IterationSummary;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::mem;

pub struct SaturationState {
    progress: Option<SaturationProgress>,
    query: Option<String>,
    hypothesis_fact_selected: Option<SelectedFact>,
    conclusion_fact_selected: Option<SelectedFact>,

    queue_entries: Vec<String>,
    last_iteration_queue_entries: Vec<String>,

    pub iterations: Vec<Iteration>,
    pub hypothesis_selected_fact_history: Vec<(String, u32)>,
}

#[derive(Clone)]
pub struct Iteration {
    pub progress: SaturationProgress,
    pub query: String,
    pub hypothesis_fact_selected: Option<SelectedFact>,
    pub conclusion_fact_selected: Option<SelectedFact>,

    pub new_queue_entries: Vec<String>,
}

#[derive(Copy, Clone)]
pub struct SaturationProgress {
    pub iteration: usize,
    pub with_conclusion_selected: usize,
    pub with_hypothesis_selected: usize,
    pub in_queue: usize,
}

#[derive(Clone, PartialEq, Eq)]
pub struct SelectedFact {
    pub fact: String,

    #[allow(dead_code)] // not yet used
    pub fact_number: Option<usize>,
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

            queue_entries: Vec::new(),
            last_iteration_queue_entries: Vec::new(),

            iterations: Vec::new(),
            hypothesis_selected_fact_history: Vec::new(),
        }
    }

    pub fn set_query(&mut self, query: String) {
        self.query = Some(query);
    }

    pub fn set_queue_entry(&mut self, _entry_number: usize, rule: String) {
        // assumes in order; which is a valid assumption
        self.queue_entries.push(rule);
    }

    pub fn set_hypothesis_fact_selected(&mut self, fact: String, fact_number: usize) {
        self.hypothesis_fact_selected = Some(SelectedFact { fact, fact_number: Some(fact_number) });
    }

    pub fn set_conclusion_fact_selected(&mut self, fact: String) {
        self.conclusion_fact_selected = Some(SelectedFact { fact, fact_number: None });
    }

    pub fn set_saturation_progress(&mut self, iteration: usize, with_conclusion_selected: usize, with_hypothesis_selected: usize, in_queue: usize) {
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

        let new_queue_entries = get_new_queue_entries(&self.last_iteration_queue_entries, &self.queue_entries);
        self.last_iteration_queue_entries = Vec::new();
        mem::swap(&mut self.last_iteration_queue_entries, &mut self.queue_entries);

        let iteration = Iteration {
            progress,
            query,
            new_queue_entries,
            hypothesis_fact_selected: self.hypothesis_fact_selected.clone(),
            conclusion_fact_selected: self.conclusion_fact_selected.clone(),
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

fn get_new_queue_entries(previous_queue: &Vec<String>, current_queue: &Vec<String>) -> Vec<String> {
    let mut current_queue_threshold = 0;
    for entry in previous_queue {
        if current_queue_threshold == current_queue.len() {
            return Vec::new();
        }

        if *entry == current_queue[current_queue_threshold] {
            current_queue_threshold += 1
        }
    }

    current_queue[current_queue_threshold..].to_vec()
}

#[test]
fn test_get_new_queue_entries() {
    let previous_queue = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let current_queue = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
    let new_queue_entries = vec!["d".to_string()];
    assert_eq!(get_new_queue_entries(&previous_queue, &current_queue), new_queue_entries);

    let current_queue = vec!["b".to_string(), "d".to_string()];
    let new_queue_entries = vec!["d".to_string()];
    assert_eq!(get_new_queue_entries(&previous_queue, &current_queue), new_queue_entries);

    let current_queue = vec!["d".to_string()];
    let new_queue_entries = vec!["d".to_string()];
    assert_eq!(get_new_queue_entries(&previous_queue, &current_queue), new_queue_entries);

    let current_queue = vec![];
    let new_queue_entries: Vec<String> = vec![];
    assert_eq!(get_new_queue_entries(&previous_queue, &current_queue), new_queue_entries);

    let previous_queue = vec![];
    let current_queue = vec!["a".to_string()];
    assert_eq!(get_new_queue_entries(&previous_queue, &current_queue), current_queue);
}
