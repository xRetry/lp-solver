use std::error::Error;
use good_lp::{constraint, solvers::highs::highs, ProblemVariables, variable, SolverModel, Solution};
use lp_solver::solver::{MySolver, my_solver};

fn solve_true() -> Result<(), Box<dyn Error>> {
    let mut problem = ProblemVariables::new();
    let a = problem.add(variable().min(0));
    let b = problem.add(variable().min(0));

    let solution = problem.maximise(7*a + 6*b)
        //.using(highs) // multiple solvers available
        .using(my_solver)
        .with(constraint!(2*a + 4*b <= 16))
        .with(constraint!(3*a + 2*b <= 12))
        .solve()?;
    println!("a={}   b={}", solution.value(a), solution.value(b));
    println!("a + b = {}", solution.eval(a + b));
    Ok(())
}

fn main() {

    solve_true().unwrap();
}
