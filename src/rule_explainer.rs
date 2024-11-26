use crate::saturation_state::Iteration;

pub struct QueryExplainer {}

impl QueryExplainer {
    pub fn get_ancestry(iterations: &[Iteration]) -> Vec<Iteration> {
        let last_iteration = iterations.last().unwrap();
        let mut ancestry: Vec<Iteration> = vec![last_iteration.clone()];

        let mut candidate = &last_iteration.query;

        for i in (0..iterations.len()).rev() {
            let iteration = &iterations[i];

            for new_queue_entry in &iteration.new_queue_entries {
                if *new_queue_entry == *candidate {
                    ancestry.push(iteration.clone());

                    candidate = &iteration.query;
                    break;
                }
            }
        }

        ancestry
    }
}

#[test]
fn test_get_new_queue_entries() {
    let progress = crate::saturation_state::SaturationProgress {
        iteration: 0,
        in_queue: 2,
        with_hypothesis_selected: 0,
        with_conclusion_selected: 0,
    };
    let iteration0 = Iteration {
        new_queue_entries: vec!["a".to_string(), "b".to_string()],
        query: "c".to_string(),
        progress,
        conclusion_fact_selected: None,
        hypothesis_fact_selected: None,
    };

    let progress = crate::saturation_state::SaturationProgress {
        iteration: 1,
        in_queue: 2,
        with_hypothesis_selected: 0,
        with_conclusion_selected: 0,
    };
    let iteration1 = Iteration {
        new_queue_entries: vec!["d".to_string()],
        query: "a".to_string(),
        progress,
        conclusion_fact_selected: None,
        hypothesis_fact_selected: None,
    };

    let progress = crate::saturation_state::SaturationProgress {
        iteration: 2,
        in_queue: 1,
        with_hypothesis_selected: 0,
        with_conclusion_selected: 0,
    };
    let iteration2 = Iteration {
        new_queue_entries: vec![],
        query: "b".to_string(),
        progress,
        conclusion_fact_selected: None,
        hypothesis_fact_selected: None,
    };

    let progress = crate::saturation_state::SaturationProgress {
        iteration: 3,
        in_queue: 2,
        with_hypothesis_selected: 0,
        with_conclusion_selected: 0,
    };
    let iteration3 = Iteration {
        new_queue_entries: vec!["e".to_string(), "f".to_string()],
        query: "d".to_string(),
        progress,
        conclusion_fact_selected: None,
        hypothesis_fact_selected: None,
    };

    let iterations: Vec<Iteration> = vec![iteration0, iteration1, iteration2, iteration3];
    let ancestry = vec![3, 1, 0];
    assert_eq!(QueryExplainer::get_ancestry(&iterations).iter().map(|x| x.progress.iteration).collect::<Vec<usize>>(), ancestry);
}
