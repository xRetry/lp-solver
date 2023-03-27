use std::collections::HashMap;

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
    let mut table_map = HashMap::new();
    for (var, c) in coeffs {
        let var_coeffs = table_map.entry(var).or_insert(Vec::new());
        var_coeffs.push(c);
    }

    MySolver {
        table_map,
        err,
        slack_var_rows: Vec::new(),
        rhs: vec![0.],
    }
}

pub struct MySolver {
    table_map: HashMap<Variable, Vec<f64>>,
    err: Option<ResolutionError>,
    slack_var_rows: Vec<usize>,
    rhs: Vec<f64>,

}

impl MySolver {
    fn get_table(&self) -> (Vec<Vec<f64>>, Vec<usize>, Vec<&Variable>) {
        let mut table = Vec::new();
        table.push(vec![0.; self.rhs.len()]); // TODO: Remove first column (maybe)
        let mut variables = Vec::new();
        for (var, col) in &self.table_map {
            table.push(col.clone());
            variables.push(var);
        }

        for row in &self.slack_var_rows {
            let mut slack_col = vec![0.; self.rhs.len()];
            slack_col[*row] = 1.;
            table.push(slack_col);
        }
        table.push(self.rhs.clone());
        let basic_var_cols = (self.table_map.len()..self.table_map.len()+self.slack_var_rows.len()).collect();

        (table, basic_var_cols, variables)
    }
}

impl SolverModel for MySolver {
    type Solution = MySolution;
    type Error = ResolutionError;

    fn add_constraint(&mut self, c: Constraint) -> ConstraintReference {
        self.rhs.push(-c.expression.constant());

        let coeffs: HashMap<Variable, f64> = c.expression.linear_coefficients().collect();

        for (k, v) in self.table_map.iter_mut() {
            if !coeffs.contains_key(k) {
                v.push(0.);
            }
        }

        for (var, c) in coeffs {
            let var_coeffs = self.table_map.get_mut(&var).unwrap();
            var_coeffs.push(c);
        }

        if !c.is_equality {
            self.slack_var_rows.push(self.rhs.len()-1);
        }

        ConstraintReference{index: self.rhs.len()-1}
    }

    fn solve(self) -> Result<Self::Solution, Self::Error> {
        if self.err.is_some() { return Err(self.err.unwrap()); }

        let (table, basic_var_cols, variables) = self.get_table();

        let solution = solve_simplex(table, basic_var_cols);
        let sol_map = solution.iter()
            .map(|(k, v)| (*variables[*k as usize], *v))
            .collect();

        Ok(MySolution{variable_values: sol_map})
    }

}

#[cfg(test)]
mod tests {
    use good_lp::{ProblemVariables, variable, constraint, SolverModel};
    use super::my_solver;

    #[test]
    fn setup() {
        let mut problem = ProblemVariables::new();
        let a = problem.add(variable().min(0));
        let b = problem.add(variable().min(0));

        let problem = problem.maximise(7*a + 6*b)
            .using(my_solver)
            .with(constraint!(2*a + 4*b <= 16))
            .with(constraint!(3*a + 2*b <= 12));

        let (table, basic_vars, vars) = problem.get_table();
        assert_eq!(basic_vars, [2, 3]);
        assert_eq!(table[0], [0.; 3]);
        assert!(table[1] == [7., 2., 3.] || table[1] == [6., 4., 2.]);
        assert!(table[2] == [7., 2., 3.] || table[2] == [6., 4., 2.]);
        assert_eq!(table[3], [0., 1., 0.]);
        assert_eq!(table[4], [0., 0., 1.]);
        assert_eq!(table[5], [0., 16., 12.]);
        assert!((*vars[0] == a && *vars[1] == b) || (*vars[0] == b && *vars[1] == a));

    }
}
