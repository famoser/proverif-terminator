#[derive(Debug, PartialEq, Eq)]
pub struct Circle {
    pub size: usize,
    pub repeat: usize
}

pub fn find_circles(fact_history: &Vec<(String, u32)>) -> Option<Circle> {
    let smallest_circle_size = find_smallest_circle_size(fact_history);
    if let Some(smallest_circle_size) = smallest_circle_size {
        let number_of_circles = find_number_of_circles(fact_history, smallest_circle_size);

        return Some(Circle {
            size: smallest_circle_size,
            repeat: number_of_circles
        })
    }

    None
}

fn find_smallest_circle_size(fact_history: &Vec<(String, u32)>) -> Option<usize> {
    let history_size = fact_history.len();
    let head_index = history_size - 1;
    let head = &fact_history[head_index];

    // only sensible if at least two entries
    if history_size < 2 { return None }
    let mut candidate_index = head_index - 1;

    loop {
        if fact_history[candidate_index] == *head {
            let expected_circle_size = head_index - candidate_index;

            // check for cycle
            let mut head_check = head_index - 1;
            while head_check > candidate_index && head_check >= expected_circle_size {
                if fact_history[head_check - expected_circle_size] != fact_history[head_check] {
                    break;
                }

                head_check -= 1;
            }

            // cycle found
            if head_check == candidate_index {
                return Some(expected_circle_size);
            }
        }

        if candidate_index == 0 {
            break;
        }

        candidate_index -= 1;
    }

    None
}

fn find_number_of_circles(fact_history: &Vec<(String, u32)>, circle_size: usize) -> usize {
    let history_len = fact_history.len();
    // only sensible if at least two circles entries
    if history_len < circle_size { return 0; }

    let mut head_index = history_len - 1;

    loop {
        let candidate = head_index - circle_size;
        if fact_history[candidate] != fact_history[head_index] {
            head_index += 1;
            break;
        }

        if candidate == 0 {
            break;
        }

        head_index -= 1
    }

    let correct_streak = history_len - (head_index-circle_size);
    correct_streak / circle_size
}

#[test]
fn test_circles_no_circle() {
    let fact_history = vec![("a".to_string(), 1), ("a".to_string(), 2), ("b".to_string(), 1)];
    assert_eq!(find_circles(&fact_history), None);

    let fact_history = vec![("a".to_string(), 1), ("b".to_string(), 1), ("a".to_string(), 1)];
    assert_eq!(find_circles(&fact_history), None);

    let fact_history = vec![("a".to_string(), 1), ("b".to_string(), 2), ("a".to_string(), 1)];
    assert_eq!(find_circles(&fact_history), None);
}

#[test]
fn test_circles_one_circle() {
    let fact_history = vec![("b".to_string(), 2), ("a".to_string(), 1), ("a".to_string(), 1)];
    assert_eq!(find_circles(&fact_history), Some(Circle { size: 1, repeat: 2 }));

    let fact_history = vec![("a".to_string(), 1), ("a".to_string(), 2), ("a".to_string(), 1), ("a".to_string(), 2), ("a".to_string(), 1)];
    assert_eq!(find_circles(&fact_history), Some(Circle { size: 2, repeat: 2 }));
}

#[test]
fn test_find_number_of_circles_multiple_circles() {
    let fact_history = vec![("a".to_string(), 1), ("a".to_string(), 2), ("a".to_string(), 1), ("a".to_string(), 2), ("a".to_string(), 1), ("a".to_string(), 2)];
    assert_eq!(find_circles(&fact_history), Some(Circle { size: 2, repeat: 3 }));
}
