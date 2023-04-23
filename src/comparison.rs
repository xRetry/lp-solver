use good_lp::{constraint, variable, variable::UnsolvedProblem,
    ProblemVariables, solvers::highs::highs, SolverModel, Solution, Constraint, Expression, Variable};
use crate::{custom_solver::CustomSolver, heuristics::StartHeuristic};
use std::time::Instant;
use serde::Serialize;

#[derive(serde::Serialize, Debug)]
enum UsedSolver {
    Highs,
    Custom,
}

#[derive(Clone)]
struct ProblemSummary {
    inner: UnsolvedProblem,
    constraints: Vec<Constraint>,
    weights: Vec<f64>,
    variables: Vec<Variable>,
}

#[derive(Debug, Serialize)]
pub struct SolutionSummary {
    used_solver: UsedSolver,
    weights: Vec<f64>,
    values: Vec<f64>,
    pub duration_sec: f64,
    num_evals: Option<usize>,
    start_heuristic: Option<StartHeuristic>,
}

impl SolutionSummary {
    fn new(
        used_solver: UsedSolver, prob_sum: ProblemSummary, solution: impl Solution, duration_sec: f64, 
        num_evals: Option<usize>, start_heuristic: Option<StartHeuristic>
    ) -> Self {
        let vals = prob_sum.variables.iter()
            .map(|v| solution.value(*v))
            .collect();

        SolutionSummary{
            used_solver,
            weights: prob_sum.weights, 
            values: vals,
            duration_sec,
            num_evals,
            start_heuristic,
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

pub fn compare_solvers(weight_fn: impl Fn() -> Vec<f64>, start_heuristic: Option<StartHeuristic>) -> [SolutionSummary; 2] {

    let problem = create_problem(weight_fn);

    let solution1 = run_with_highs_solver(problem.clone());
    let solution2 = run_with_custom_solver(problem, start_heuristic);

    assert!(solution1 == solution2);

    [solution1, solution2]
}

fn create_problem(weight_fn: impl Fn() -> Vec<f64>) -> ProblemSummary {
    let weights_obj = weight_fn();

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

fn run_with_highs_solver(problem: ProblemSummary) -> SolutionSummary {
    let mut solver = problem.inner.clone().using(highs);
    for c in &problem.constraints {
        solver = solver.with(c.clone());
    }

    let time_start = Instant::now();
    let solution = solver.solve().unwrap();
    let duration = time_start.elapsed();

    SolutionSummary::new(UsedSolver::Highs, problem, solution, duration.as_secs_f64(), None, None)
}

fn run_with_custom_solver(problem: ProblemSummary, start_heuristic: Option<StartHeuristic>) -> SolutionSummary {
    let mut solver = problem.inner.clone().using(CustomSolver::new);
    for c in &problem.constraints {
        solver = solver.with(c.clone());
    }

    if let Some(heu) = &start_heuristic {
        solver = solver.add_heuristic(&problem.weights, heu.clone());
    }

    let time_start = Instant::now();
    let solution = solver.solve().unwrap();
    let duration = time_start.elapsed();
    let num_evals = Some(solution.num_evals);

    SolutionSummary::new(UsedSolver::Custom, problem, solution, 
        duration.as_secs_f64(), num_evals, start_heuristic)
}
