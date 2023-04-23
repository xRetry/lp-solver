use rand::Rng;
use crate::solution::CustomSolution;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum StartHeuristic {
    Greedy,
    Random,
    EqualCount,
}

pub fn solution_from_heuristic(object_vals: &Vec<f64>, start_heuristic: StartHeuristic) -> Option<CustomSolution> {
    let values = match start_heuristic {
        StartHeuristic::Greedy => greedy_heuristic(object_vals),
        StartHeuristic::Random => random_heuristic(object_vals),
        StartHeuristic::EqualCount => equal_heuristic(object_vals),
    };

    Some(CustomSolution::new(values))
}

fn greedy_heuristic(object_vals: &Vec<f64>) -> Vec<f64> {
    let mut idx_sorted: Vec<_> = (0..object_vals.len()).collect();
    idx_sorted.sort_by(|&i, &j| object_vals[j].partial_cmp(&object_vals[i]).unwrap());

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
    solution[object_vals.len()] = (sum_left - sum_right).abs();
    solution
}

fn random_heuristic(object_vals: &Vec<f64>) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut solution: Vec<_> = (0..object_vals.len()+1)
        .map(|_| rng.gen_bool(0.5) as i32 as f64)
        .collect();

    solution[object_vals.len()] = object_vals.iter().enumerate()
        .map(|(i, v)| (2.*solution[i] - 1.) * v)
        .sum::<f64>().abs();
    solution
}

fn equal_heuristic(object_vals: &Vec<f64>) -> Vec<f64> {
    let mut solution = vec![0.; object_vals.len()+1];
    solution.iter_mut() 
        .skip(object_vals.len() / 2)
        .for_each(|s| *s = 1.);

    solution[object_vals.len()] = object_vals.iter().enumerate()
        .map(|(i, v)| (2.*solution[i] - 1.) * v)
        .sum::<f64>().abs();
    solution
}

#[cfg(test)]
mod tests { 
    use super::greedy_heuristic;
    use super::equal_heuristic;

    #[test]
    fn test_greedy1() {
        let vals = vec![2., 5., 3.];
        let sol = greedy_heuristic(&vals);
        assert!(sol[0] < 1e-6);
        assert!(sol[1] - 1. < 1e-6);
        assert!(sol[2] < 1e-6);
        assert!(sol[3] < 1e-6);
    }

    #[test]
    fn test_greedy2() {
        let vals = vec![3., 3., 2., 2., 2.];
        let sol = greedy_heuristic(&vals);
        assert!(sol[0] - 1. < 1e-6);
        assert!(sol[1] < 1e-6);
        assert!(sol[2] - 1. < 1e-6);
        assert!(sol[3] < 1e-6);
        assert!(sol[4] - 1. < 1e-6);
        assert!(sol[5] - 2. < 1e-6);
    }

    #[test]
    fn test_equal() {
        let vals = vec![1., 1., 2., 2.];
        let sol = equal_heuristic(&vals);
        assert!(sol[0] < 1e-6);
        assert!(sol[1] < 1e-6);
        assert!(sol[2] - 1. < 1e-6);
        assert!(sol[3] - 1. < 1e-6);
        assert!(sol[4] - 2. < 1e-6);
    }
}
