use std::error::Error;
use good_lp::{constraint, solvers::highs::highs, ProblemVariables, variable, SolverModel, Solution};
use lp_solver::solver::MySolver;


fn solve_true() -> Result<(), Box<dyn Error>> {
    let mut problem = ProblemVariables::new();
    let a = problem.add(variable().min(0));
    let b = problem.add(variable().min(0));
    let solution = problem.maximise(7*a + 6*b)
        .using(highs) // multiple solvers available
        .with(constraint!(2*a + 4*b <= 16))
        .with(constraint!(3*a + 2*b <= 12))
        .solve()?;
    println!("a={}   b={}", solution.value(a), solution.value(b));
    println!("a + b = {}", solution.eval(a + b));
    Ok(())
}

fn main() {

    let obj = vec![7., 6., 0., 0.];
    let mut table = vec![
        vec![2., 4., 1., 0.],
        vec![3., 2., 0., 1.],
    ];
    let mut c_b = vec![0., 0.];
    let mut b = vec![16., 12.];

    solve_true().unwrap();

    println!("{:?}", b);

}
