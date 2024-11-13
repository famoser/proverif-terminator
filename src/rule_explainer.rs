use crate::saturation_state::Iteration;

pub struct QueryExplainer {}

impl QueryExplainer {
    pub fn new() -> Self {
        QueryExplainer {}
    }

    pub fn get_ancestry(&self, iterations: &[Iteration]) -> Vec<Iteration> {
        let mut ancestry: Vec<Iteration> = Vec::new();

        let mut skip_queue_entries = 0;

        for iteration in iterations.iter().rev() {
            let new_queue_entries = iteration.new_queue_entries.len();

            if skip_queue_entries < new_queue_entries || ancestry.is_empty() {
                ancestry.push(iteration.clone());
                skip_queue_entries = iteration.progress.in_queue;
            }

            skip_queue_entries -= new_queue_entries
        }

        ancestry
    }
}
