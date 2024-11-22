use crate::saturation_state::Iteration;

pub struct QueryExplainer {}

impl QueryExplainer {
    pub fn new() -> Self {
        QueryExplainer {}
    }

    pub fn get_ancestry(&self, iterations: &[Iteration]) -> Vec<Iteration> {
        let mut ancestry: Vec<Iteration> = Vec::new();

        let mut skip_queue_entries = 0;

        for i in (0..iterations.len()).rev() {
            let iteration = &iterations[i];
            let new_queue_entries = iteration.new_queue_entries.len();

            if skip_queue_entries < new_queue_entries || ancestry.is_empty() {
                if i == 0 {
                    break;
                }

                ancestry.push(iteration.clone());

                // need to take from previous iteration, because in case of deriving bad, the queue is emptied
                skip_queue_entries = iterations[i - 1].progress.in_queue - 1;
                continue;
            }

            skip_queue_entries -= new_queue_entries
        }

        ancestry
    }
}
