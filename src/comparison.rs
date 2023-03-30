use good_lp::{constraint, variable, variable::UnsolvedProblem,
    ProblemVariables, solvers::highs::highs, SolverModel, Solution, Constraint, Solver, Expression, Variable};
use crate::integer_solver::IntegerSolver;
use std::time::{Instant, Duration};
use rand::Rng;

#[derive(Clone)]
struct ProblemSummary {
    inner: UnsolvedProblem,
    constraints: Vec<Constraint>,
    weights: Vec<f64>,
    variables: Vec<Variable>,
}

#[derive(Debug)]
pub struct SolutionSummary {
    weights: Vec<f64>,
    values: Vec<f64>,
    duration: Duration,
}

impl SolutionSummary {
    fn new(prob_sum: ProblemSummary, solution: impl Solution, duration: Duration) -> Self {
        let vals = prob_sum.variables.iter()
            .map(|v| solution.value(*v))
            .collect();

        SolutionSummary{
            weights: prob_sum.weights, 
            values: vals,
            duration,
        }
    }
}

impl PartialEq for SolutionSummary {
    fn eq(&self, other: &Self) -> bool {
        if self.values.len() != other.values.len() {
            return false;
        }

        let count = self.values.iter().zip(&other.values)
            .map(|(v1, v2)| (v1 - 1. < 10e-6) ^ (v2 - 1. < 10e-6))
            .count();

        count == 0 || count == self.values.len()
    }
}

pub fn compare_solvers(num_vars: usize) -> [SolutionSummary; 2] {

    let problem = create_problem(num_vars);

    let solution1 = run_with_solver(highs, problem.clone());
    let solution2 = run_with_solver(IntegerSolver::new, problem);

    assert!(solution1 == solution2);

    [solution1, solution2]
}

fn create_problem(num_vars: usize) -> ProblemSummary {
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

    ProblemSummary {
        inner: problem,
        variables: vars,
        constraints: constr,
        weights: weights_obj,
    }
}

fn run_with_solver<S: Solver>(solver: S, problem: ProblemSummary) -> SolutionSummary {
    let mut solver = problem.inner.clone().using(solver);
    for c in &problem.constraints {
        solver = solver.with(c.clone());
    }

    let time_start = Instant::now();
    let solution = solver.solve().unwrap();
    let duration = time_start.elapsed();

    SolutionSummary::new(problem, solution, duration)
}
