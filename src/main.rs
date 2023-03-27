use std::error::Error;
use good_lp::{constraint, variable, variable::UnsolvedProblem,
    ProblemVariables, solvers::highs::highs, SolverModel, Solution, Constraint, Solver};
use lp_solver::integer_solver::IntegerSolver;

fn solve_int() -> Result<(), Box<dyn Error>> {
    let mut problem = ProblemVariables::new();
    let a = problem.add(variable().integer().max(5.5));
    let b = problem.add(variable().max(5.5));
    let problem = problem.maximise(a + b);

    let constraints = vec![
        constraint!(a + b <= 20),
    ];

    let sol1 = run_with_solver(highs, problem.clone(), constraints.clone());
    let sol2 = run_with_solver(IntegerSolver::new, problem, constraints);

    assert!(sol1.value(a) == sol2.value(a));
    assert!(sol2.value(b) == sol2.value(b));
    Ok(())
}

fn run_with_solver<S: Solver>(solver: S, problem: UnsolvedProblem, constraints: Vec<Constraint>) -> impl Solution {
    let mut solver = problem.clone().using(solver);
    for c in constraints {
        solver = solver.with(c);
    }
    solver.solve().unwrap()
}


fn main() {

    solve_int().unwrap();
}
