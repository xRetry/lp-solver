use lp_solver::comparison::{SolutionSummary, compare_solvers};

fn main() {
    let num_elems = [3, 5, 10];

    let summaries: Vec<[SolutionSummary; 2]> = num_elems.into_iter()
        .map(compare_solvers)
        .collect();

    dbg!(summaries);
}
