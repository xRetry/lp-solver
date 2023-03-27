use std::collections::HashSet;
use good_lp::{constraint, solvers::highs::{highs, HighsSolution}, 
    variable::UnsolvedProblem, SolverModel, 
    Solution, Constraint, Variable, ResolutionError
};

pub struct IntegerSolver {
    problem: UnsolvedProblem,
    constraints: Vec<Constraint>,
    int_vars: HashSet<Variable>,
    best_solution: Option<HighsSolution>,
}

impl IntegerSolver {
    pub fn new(mut problem: UnsolvedProblem) -> Self {

        let int_vars: HashSet<Variable> = problem.variables.iter_variables_with_def()
            .filter(|(_, vd)| vd.is_integer)
            .map(|(v, _)| v)
            .collect();

        for v in problem.variables.variables.iter_mut() {
            v.is_integer = false;
        }
        
        IntegerSolver { 
            problem,
            constraints: Vec::new(),
            int_vars,
            best_solution: None,
        }
    }

    fn solve_rec(&mut self, mut constraints: Vec<Constraint>) {
        let objective = &self.problem.objective;

        let mut solver = self.problem.clone().using(highs);
        for c in &constraints {
            solver = solver.with(c.clone());
        }

        // Current solution found or stop
        let Ok(cur_solution) = solver.solve() else {
            return;
        };
        let objective_value = cur_solution.eval(objective);

        let next_var = self.problem.variables.iter_variables_with_def()
            .map(|(v, _)| v)
            .filter(|v| self.int_vars.contains(v))
            .filter(|v| cur_solution.value(*v).fract() != 0.)
            .reduce(|v1, v2| if cur_solution.value(v1) > cur_solution.value(v2) { v1 } else { v2 });
        
        // Current solution worse than best -> stop
        if let Some(best_solution) = &self.best_solution {
            if objective_value < objective.eval_with(best_solution) { return; }
        }

        // At this point, the current solution is better
        // -> override best if no next integer variable
        let Some(next) = next_var else { 
            self.best_solution = Some(cur_solution);
            return;
        };

        // Preparing next recursion
        let mut constraints_lower = constraints.clone();
        constraints_lower.push(constraint!(next <= cur_solution.value(next).floor()));
        constraints.push(constraint!(next >= cur_solution.value(next).ceil()));

        // Recursive calls
        self.solve_rec(constraints_lower);
        self.solve_rec(constraints);
    }
}

impl SolverModel for IntegerSolver {
    type Solution = HighsSolution;
    type Error = ResolutionError;

    fn solve(mut self) -> Result<Self::Solution, Self::Error> {
        self.solve_rec(self.constraints.clone());
        self.best_solution.ok_or(ResolutionError::Other("No Solution found"))
    }

    fn add_constraint(&mut self, c: Constraint) -> constraint::ConstraintReference {
        self.constraints.push(c);
        constraint::ConstraintReference { index: self.constraints.len()-1 }
    }
}

#[cfg(test)]
mod tests {
    use good_lp::{ProblemVariables, variable, constraint, SolverModel};

    use super::IntegerSolver;

    #[test]
    fn test_setup() {
        let mut problem = ProblemVariables::new();
        let a = problem.add(variable().integer().min(0));
        let b = problem.add(variable().min(0));
        let solver = problem.maximise(a + b).using(IntegerSolver::new);

        let constr = constraint!(b >= 10);
        let solver = solver.with(constr);

        assert!(solver.int_vars.len() == 1 && solver.int_vars.contains(&a));
        assert!(solver.problem.variables.iter_variables_with_def()
            .filter(|(_, v)| v.is_integer).count() == 0)
    }
}
