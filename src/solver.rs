
use good_lp::{SolverModel, constraint::ConstraintReference, Constraint, 
    solvers::{ResolutionError, ObjectiveDirection}, variable::UnsolvedProblem, IntoAffineExpression, Variable };

use crate::solution::MySolution;
use crate::algorithm::solve_simplex;

pub fn my_solver(to_solve: UnsolvedProblem) -> MySolver {
    let err = match to_solve.direction {
        ObjectiveDirection::Minimisation => Some(ResolutionError::Other("Minimization not valid")),
        ObjectiveDirection::Maximisation => None,
    };

    let coeffs = to_solve.objective.linear_coefficients();
    let mut variables = Vec::new();
    let mut table = vec![Vec::new()];
    for (var, c) in coeffs {
        variables.push(var);
        table[0].push(c);
    }

    MySolver {
        table,
        variables,
        err,
    }
}

pub struct MySolver {
    table: Vec<Vec<f64>>,
    variables: Vec<Variable>,
    err: Option<ResolutionError>,
}

impl SolverModel for MySolver {
    type Solution = MySolution;
    type Error = ResolutionError;

    fn add_constraint(&mut self, c: Constraint) -> ConstraintReference {
        //let coeffs = c.expression.linear_coefficients();
        todo!();
    }

    fn solve(self) -> Result<Self::Solution, Self::Error> {
        if self.err.is_some() { return Err(self.err.unwrap()); }

        //solve_simplex(self.table, self.var_names);
        todo!();
    }
}

