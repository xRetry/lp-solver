use serde_json;
use lp_solver::{comparison::compare_solvers, 
    weight_functions::{random_distribution, equal_distribution, linear_distribution},
    heuristics::StartHeuristic,
};

fn main() {
    let num_repeats = 10;
    let min_weight = 1.;
    let max_weight = 100.;

    let mut solutions = Vec::new();
    for num_vars in (5..=19).step_by(2) {
        for i in 0..num_repeats {
            print!("Vars: {}, Iter: {}/{} ... ", num_vars, i, num_repeats);
            let weights_fn = || random_distribution(num_vars, min_weight, max_weight);
            //let weights_fn = || linear_distribution(num_vars, min_weight, max_weight);
            //let weights_fn = || equal_distribution(num_vars, max_weight);
            let sols = compare_solvers(weights_fn, Some(StartHeuristic::EqualCount));

            let duration = sols[1].duration_sec;
            println!("{:?}", duration);

            solutions.push(sols);
        }

    }

    let solutions: Vec<_> = solutions.iter().flatten().collect();

    std::fs::write(
        "data/data_random_equal.json",
        serde_json::to_string_pretty(&solutions).unwrap()
    ).unwrap();
}
