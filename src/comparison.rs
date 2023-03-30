use good_lp::{constraint, variable, variable::UnsolvedProblem,
    ProblemVariables, solvers::highs::highs, SolverModel, Solution, Constraint, Expression, Variable};
use crate::custom_solver::CustomSolver;
use std::time::{Instant, Duration};
use serde::ser::{Serialize, Serializer, SerializeStruct};

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

#[derive(Debug)]
pub struct SolutionSummary {
    used_solver: UsedSolver,
    weights: Vec<f64>,
    values: Vec<f64>,
    duration: Duration,
    num_evals: Option<usize>
}

impl SolutionSummary {
    fn new(used_solver: UsedSolver, prob_sum: ProblemSummary, solution: impl Solution, duration: Duration, num_evals: Option<usize>) -> Self {
        let vals = prob_sum.variables.iter()
            .map(|v| solution.value(*v))
            .collect();

        SolutionSummary{
            used_solver,
            weights: prob_sum.weights, 
            values: vals,
            duration,
            num_evals
        }
    }
}

impl Serialize for SolutionSummary {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("SolutionSummary", 3)?;
        s.serialize_field("used_solver", &self.used_solver)?;
        s.serialize_field("num_vars", &self.weights.len())?;
        s.serialize_field("duration", &self.duration.as_secs_f64())?;
        s.serialize_field("num_evals", &self.num_evals)?;
        s.end()
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

pub fn compare_solvers(weight_fn: impl Fn() -> Vec<f64>) -> [SolutionSummary; 2] {

    let problem = create_problem(weight_fn);

    let solution1 = run_with_highs_solver(problem.clone());
    let solution2 = run_with_custom_solver(problem);

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

    SolutionSummary::new(UsedSolver::Highs, problem, solution, duration, None)
}

fn run_with_custom_solver(problem: ProblemSummary) -> SolutionSummary {
    let mut solver = problem.inner.clone().using(CustomSolver::new);
    for c in &problem.constraints {
        solver = solver.with(c.clone());
    }

    let time_start = Instant::now();
    let solution = solver.solve().unwrap();
    let duration = time_start.elapsed();
    let num_evals = Some(solution.num_evals);

    SolutionSummary::new(UsedSolver::Custom, problem, solution, 
        duration, num_evals)
}
