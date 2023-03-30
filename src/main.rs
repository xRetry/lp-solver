use lp_solver::{comparison::{SolutionSummary, compare_solvers}, 
    weight_functions::random_distribution
};

fn main() {
    let num_vars = [3, 5, 10];

    let summaries: Vec<SolutionSummary> = num_vars.into_iter()
        .map(|n| move || random_distribution(n, 0., 100.))
        .map(compare_solvers)
        .flatten()
        .collect();

    dbg!(summaries);
}
