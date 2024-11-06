#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Cycle {
    pub size: usize,
    pub repeat: usize,
}

pub fn find_cycles(fact_history: &[(String, u32)]) -> Option<Cycle> {
    let smallest_cycle_size = find_smallest_cycle_size(fact_history);
    if let Some(smallest_cycle_size) = smallest_cycle_size {
        let number_of_cycles = find_number_of_cycles(fact_history, smallest_cycle_size);

        return Some(Cycle {
            size: smallest_cycle_size,
            repeat: number_of_cycles,
        });
    }

    None
}

fn find_smallest_cycle_size(fact_history: &[(String, u32)]) -> Option<usize> {
    let history_size = fact_history.len();
    let head_index = history_size - 1;
    let head = &fact_history[head_index];

    // only sensible if at least two entries
    if history_size < 2 {
        return None;
    }
    let mut candidate_index = head_index - 1;

    loop {
        if fact_history[candidate_index] == *head {
            let expected_cycle_size = head_index - candidate_index;

            // check for cycle
            let mut head_check = head_index - 1;
            while head_check > candidate_index && head_check >= expected_cycle_size {
                if fact_history[head_check - expected_cycle_size] != fact_history[head_check] {
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

fn find_number_of_cycles(fact_history: &[(String, u32)], cycle_size: usize) -> usize {
    let history_len = fact_history.len();
    // only sensible if at least two cycles entries
    if history_len < cycle_size {
        return 0;
    }

    let mut head_index = history_len - 1;

    loop {
        let candidate = head_index - cycle_size;
        if fact_history[candidate] != fact_history[head_index] {
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
    let fact_history = vec![
        ("a".to_string(), 1),
        ("a".to_string(), 2),
        ("b".to_string(), 1),
    ];
    assert_eq!(find_cycles(&fact_history), None);

    let fact_history = vec![
        ("a".to_string(), 1),
        ("b".to_string(), 1),
        ("a".to_string(), 1),
    ];
    assert_eq!(find_cycles(&fact_history), None);

    let fact_history = vec![
        ("a".to_string(), 1),
        ("b".to_string(), 2),
        ("a".to_string(), 1),
    ];
    assert_eq!(find_cycles(&fact_history), None);
}

#[test]
fn test_cycles_one_cycle() {
    let fact_history = vec![
        ("b".to_string(), 2),
        ("a".to_string(), 1),
        ("a".to_string(), 1),
    ];
    assert_eq!(
        find_cycles(&fact_history),
        Some(Cycle { size: 1, repeat: 2 })
    );

    let fact_history = vec![
        ("a".to_string(), 1),
        ("a".to_string(), 2),
        ("a".to_string(), 1),
        ("a".to_string(), 2),
        ("a".to_string(), 1),
    ];
    assert_eq!(
        find_cycles(&fact_history),
        Some(Cycle { size: 2, repeat: 2 })
    );
}

#[test]
fn test_find_number_of_cycles_multiple_cycles() {
    let fact_history = vec![
        ("a".to_string(), 1),
        ("a".to_string(), 2),
        ("a".to_string(), 1),
        ("a".to_string(), 2),
        ("a".to_string(), 1),
        ("a".to_string(), 2),
    ];
    assert_eq!(
        find_cycles(&fact_history),
        Some(Cycle { size: 2, repeat: 3 })
    );
}
