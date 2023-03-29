use good_lp::{constraint, variable, variable::UnsolvedProblem,
    ProblemVariables, solvers::highs::highs, SolverModel, Solution, Constraint, Solver, Expression, Variable};
use lp_solver::integer_solver::IntegerSolver;
use std::time::{Instant, Duration};
use rand::Rng;

#[derive(Debug)]
struct SolutionSummary {
    weights: Vec<f64>,
    duration: Duration,
}

fn compare_solvers(num_vars: usize) -> [SolutionSummary; 2] {

    let (problem, constr, weights) = create_problem(num_vars);

    let (sol1, dur1) = run_with_solver(highs, problem.clone(), constr.clone());
    let (sol2, dur2) = run_with_solver(IntegerSolver::new, problem.clone(), constr);

    dbg!(problem.variables.iter_variables_with_def()
        .map(|(v, _)| [sol1.value(v), sol2.value(v)])
        .collect::<Vec<[f64; 2]>>()
    );

    // TODO: Collect both arrays into bits and perform XOR (^ operator)
    // All bits need to be 0 to pass assert
    assert!(problem.variables.iter_variables_with_def()
        .all(|(v, _)| sol1.value(v) - sol2.value(v) < 10e-6));

    [
        SolutionSummary{weights: weights.clone(), duration: dur1}, 
        SolutionSummary{weights, duration: dur2}
    ]
}

fn create_problem(num_vars: usize) -> (UnsolvedProblem, Vec<Constraint>, Vec<f64>) {
    let mut rng = rand::thread_rng();
    let weights_obj: Vec<f64> = (0..num_vars)
        .map(|_| rng.gen_range(0.0..100.))
        .collect();

    let mut problem = ProblemVariables::new();

    let vars: Vec<Variable> = (0..weights_obj.len())
        .map(|_| problem.add(variable().integer().min(0).max(1)))
        .collect();
    let diff = problem.add(variable());

    let constr_weights: Expression = weights_obj.iter()
        .zip(&vars)
        .map(|(w, v)| (2 * *v - 1) * *w)
        .sum();

    let problem = problem.minimise(diff);
    let constr = vec![
        constraint!(constr_weights.clone() <= diff),
        constraint!(-constr_weights <= diff)
    ];

    (problem, constr, weights_obj)
}

fn run_with_solver<S: Solver>(solver: S, problem: UnsolvedProblem, constraints: Vec<Constraint>) 
    -> (impl Solution, Duration) {
    let mut solver = problem.clone().using(solver);
    for c in constraints {
        solver = solver.with(c);
    }

    let time_start = Instant::now();
    let solution = solver.solve().unwrap();
    let duration = time_start.elapsed();

    (solution, duration)
}

fn main() {
    let num_elems = [3, 5, 10];

    let summaries: Vec<[SolutionSummary; 2]> = num_elems.into_iter()
        .map(compare_solvers)
        .collect();

    dbg!(summaries);
}
