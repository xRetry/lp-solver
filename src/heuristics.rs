use crate::solution::CustomSolution;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StartHeuristic {
    Greedy,
}

pub fn solution_from_heuristic(object_vals: &Vec<f64>, start_heuristic: StartHeuristic) -> Option<CustomSolution> {
    let values = match start_heuristic {
        StartHeuristic::Greedy => greedy_heuristic(object_vals),
    };

    Some(CustomSolution::new(values))
}

fn greedy_heuristic(object_vals: &Vec<f64>) -> Vec<f64> {
    let mut idx_sorted: Vec<_> = (0..object_vals.len()).collect();
    idx_sorted.sort_by(|&i, &j| object_vals[i].partial_cmp(&object_vals[j]).unwrap());

    let mut solution = vec![0.; object_vals.len()+1];
    let (mut sum_left, mut sum_right) = (0., 0.);
    for idx in idx_sorted {
        if sum_left < sum_right {
            sum_left += object_vals[idx];
        } else {
            sum_right += object_vals[idx];
            solution[idx] = 1.;
        }
    }

    solution
}

#[cfg(test)]
mod tests {

}
