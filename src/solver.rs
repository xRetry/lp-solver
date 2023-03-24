
use good_lp::{SolverModel, constraint::ConstraintReference, Constraint, 
    solvers::{ResolutionError, ObjectiveDirection}, variable::UnsolvedProblem };

use crate::solution::MySolution;
use crate::algorithm::solve_simplex;

pub fn my_solver<'a>(to_solve: UnsolvedProblem) -> MySolver<'a> {
    //let UnsolvedProblem {
    //    objective,
    //} = to_solve;

    //let coeffs = objective.linear_coefficients();

    //let err = match direction {
    //    ObjectiveDirection::Minimisation => Some(ResolutionError::Other("Minimization not valid")),
    //    ObjectiveDirection::Maximisation => None,
    //};
    
    todo!();

}

pub struct MySolver<'a> {
    table: Vec<Vec<f64>>,
    col_names: Vec<String>,
    row_names: Vec<&'a String>,
    err: Option<ResolutionError>,
}

impl<'a> SolverModel for MySolver<'a> {
    type Solution = MySolution;
    type Error = ResolutionError;

    fn add_constraint(&mut self, c: Constraint) -> ConstraintReference {
        todo!();
    }

    fn solve(self) -> Result<Self::Solution, Self::Error> {
        if self.err.is_some() { return Err(self.err.unwrap()); }

        //solve_simplex(self.table, self.var_names);
        todo!();
    }
}

