use crate::iteration_summary::IterationSummary;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Cycle {
    pub size: usize,
    pub repeat: usize,
}

pub struct CycleDetector {
    last_cycle: Option<Cycle>,
    last_cycle_end: usize,

    last_history_length: usize,
}

impl CycleDetector {
    pub fn new() -> Self {
        CycleDetector {
            last_cycle: None,
            last_cycle_end: 0,

            last_history_length: 0,
        }
    }

    pub fn check_cycles(&mut self, history: &[(String, u32)], printer: &mut IterationSummary) {
        // early out if no change in history
        // this assumes that the passed fact histories are related.
        if self.last_history_length == history.len() {
            return;
        }

        // early-out if cycle potentially still active (avoids spamming smaller cycles in big cycle)
        if self.last_cycle.is_some() {
            if history.len() < self.last_cycle_end {
                return;
            }

            self.last_cycle = None;
        }

        if let Some(cycle) = find_cycles(history) {
            self.last_cycle = Some(cycle);
            self.last_cycle_end = history.len() + cycle.size;

            if cycle.size * cycle.repeat > 1000 {
                printer.add_error("Cycle".to_string(), format!("{:?}", cycle));
                return;
            }

            if cycle.size * cycle.repeat > 100 {
                printer.add_warning("Cycle".to_string(), format!("{:?}", cycle));
                return;
            }

            if cycle.size * cycle.repeat > 10 {
                printer.add_info("Cycle".to_string(), format!("{:?}", cycle));
            }
        }
    }
}

fn find_cycles(history: &[(String, u32)]) -> Option<Cycle> {
    let smallest_cycle_size = find_smallest_cycle_size(history);
    if let Some(smallest_cycle_size) = smallest_cycle_size {
        let number_of_cycles = find_number_of_cycles(history, smallest_cycle_size);

        return Some(Cycle {
            size: smallest_cycle_size,
            repeat: number_of_cycles,
        });
    }

    None
}

fn find_smallest_cycle_size(history: &[(String, u32)]) -> Option<usize> {
    let history_size = history.len();
    let head_index = history_size - 1;
    let head = &history[head_index];

    // only sensible if at least two entries
    if history_size < 2 {
        return None;
    }
    let mut candidate_index = head_index - 1;

    loop {
        if history[candidate_index] == *head {
            let expected_cycle_size = head_index - candidate_index;

            // check for cycle
            let mut head_check = head_index - 1;
            while head_check > candidate_index && head_check >= expected_cycle_size {
                if history[head_check - expected_cycle_size] != history[head_check] {
                    break;
                }

                head_check -= 1;
            }

            // cycle found
            if head_check == candidate_index {
                return Some(expected_cycle_size);
            }
        }

        if candidate_index == 0 {
            break;
        }

        candidate_index -= 1;
    }

    None
}

fn find_number_of_cycles(history: &[(String, u32)], cycle_size: usize) -> usize {
    let history_len = history.len();
    // only sensible if at least two cycles entries
    if history_len < cycle_size {
        return 0;
    }

    let mut head_index = history_len - 1;

    loop {
        let candidate = head_index - cycle_size;
        if history[candidate] != history[head_index] {
            head_index += 1;
            break;
        }

        if candidate == 0 {
            break;
        }

        head_index -= 1
    }

    let correct_streak = history_len - (head_index - cycle_size);
    correct_streak / cycle_size
}

#[test]
fn test_cycles_no_cycle() {
    let history = vec![
        ("a".to_string(), 1),
        ("a".to_string(), 2),
        ("b".to_string(), 1),
    ];
    assert_eq!(find_cycles(&history), None);

    let history = vec![
        ("a".to_string(), 1),
        ("b".to_string(), 1),
        ("a".to_string(), 1),
    ];
    assert_eq!(find_cycles(&history), None);

    let history = vec![
        ("a".to_string(), 1),
        ("b".to_string(), 2),
        ("a".to_string(), 1),
    ];
    assert_eq!(find_cycles(&history), None);
}

#[test]
fn test_cycles_one_cycle() {
    let history = vec![
        ("b".to_string(), 2),
        ("a".to_string(), 1),
        ("a".to_string(), 1),
    ];
    assert_eq!(find_cycles(&history), Some(Cycle { size: 1, repeat: 2 }));

    let history = vec![
        ("a".to_string(), 1),
        ("a".to_string(), 2),
        ("a".to_string(), 1),
        ("a".to_string(), 2),
        ("a".to_string(), 1),
    ];
    assert_eq!(find_cycles(&history), Some(Cycle { size: 2, repeat: 2 }));
}

#[test]
fn test_find_number_of_cycles_multiple_cycles() {
    let history = vec![
        ("a".to_string(), 1),
        ("a".to_string(), 2),
        ("a".to_string(), 1),
        ("a".to_string(), 2),
        ("a".to_string(), 1),
        ("a".to_string(), 2),
    ];
    assert_eq!(find_cycles(&history), Some(Cycle { size: 2, repeat: 3 }));
}
