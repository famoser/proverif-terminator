pub fn find_circle_size(fact_history: &Vec<(String, u32)>) -> Option<usize> {
    let history_size = fact_history.len();
    let head_index = history_size - 1;
    let head = &fact_history[head_index];

    // only sensible if at least two entries
    if history_size < 2 { return None }
    let mut candidate = history_size - 2;
    let min_candidate = history_size / 2;

    while candidate > min_candidate {
        if fact_history[candidate] == *head {
            // check for cycle
            let mut candidate_check = candidate - 1;
            let mut head_check = head_index - 1;
            while head_check > candidate {
                if fact_history[candidate_check] != fact_history[head_check] {
                    break;
                }

                candidate_check -= 1;
                head_check -= 1;
            }

            // cycle found
            if head_check == candidate {
                return Some(history_size - candidate - 1);
            }
        }

        candidate -= 1;
    }

    None
}