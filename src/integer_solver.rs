use std::collections::HashSet;
use good_lp::{constraint, solvers::highs::{highs, HighsSolution}, 
    variable::UnsolvedProblem, SolverModel, 
    Solution, Constraint, Variable, ResolutionError,
    solvers::ObjectiveDirection,
};

pub struct IntegerSolver {
    problem: UnsolvedProblem,
    constraints: Vec<Constraint>,
    int_vars: HashSet<Variable>,
    best_solution: Option<HighsSolution>,
    fn_is_better: fn(f64, f64) -> bool,
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

        let fn_is_better = match problem.direction {
            ObjectiveDirection::Maximisation => |cur, best| cur > best,
            ObjectiveDirection::Minimisation => |cur, best| cur < best,
        };

        IntegerSolver { 
            problem,
            constraints: Vec::new(),
            int_vars,
            best_solution: None,
            fn_is_better,
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
            if !(self.fn_is_better)(objective_value, objective.eval_with(best_solution)) { return; }
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
    use good_lp::{ProblemVariables, variable, constraint, SolverModel, Solution, Variable, Expression};

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

    #[test]
    fn test_minimization() {
        let mut problem = ProblemVariables::new();
        let x = problem.add(variable().integer().min(5.5));
        let solution = problem.minimise(x).using(IntegerSolver::new)
            .solve().unwrap();

        assert!(solution.value(x) - 6. < 10e-6);
    }

    #[test]
    fn test_maximization() {
        let mut problem = ProblemVariables::new();
        let x = problem.add(variable().integer().max(5.5));
        let solution = problem.maximise(x).using(IntegerSolver::new)
            .solve().unwrap();

        assert!(solution.value(x) - 5. < 10e-6);
    }

    #[test]
    fn test_example1() {
        let mut problem = ProblemVariables::new();
        let x1 = problem.add(variable().integer().min(0));
        let x2 = problem.add(variable().integer().min(0));
        let solution = problem.maximise(5*x1 + 8*x2).using(IntegerSolver::new)
            .with(constraint!(x1 + x2 <= 6))
            .with(constraint!(5*x1 + 9*x2 <= 45))
            .solve().unwrap();

        assert!(solution.value(x1) - 0. < 10e-6);
        assert!(solution.value(x1) - 5. < 10e-6);
    }

    #[test]
    fn test_example2() {
        let mut problem = ProblemVariables::new();
        let c = problem.add(variable().integer().min(0));
        let t = problem.add(variable().integer().min(0));
        let solution = problem.maximise(12*c + 13*t).using(IntegerSolver::new)
            .with(constraint!(6*c + 7*t <= 21))
            .solve().unwrap();

        assert!(solution.value(c) - 0. < 10e-6);
        assert!(solution.value(t) - 3. < 10e-6);
    }

    #[test]
    fn test_example3() {
        let weights_obj = [0.1, 0.04, 0.07, 0.12, 0.08, 0.03, 0.04, 0.05];
        let weights_constr = [9., 7., 5., 8., 5., 3., 7., 10.];

        let mut problem = ProblemVariables::new();
        let vars: Vec<Variable> = (0..weights_obj.len())
            .map(|_| problem.add(variable().integer().min(0).max(1)))
            .collect();

        let obj: Expression = weights_obj.iter()
            .zip(&vars)
            .map(|(w, v)| *w * *v)
            .sum();

        let constr_lhs: Expression = weights_constr.iter()
            .zip(&vars)
            .map(|(w, v)| *w * *v)
            .sum();

        let solution = problem.maximise(obj).using(IntegerSolver::new)
            .with(constraint!(constr_lhs <= 40))
            .solve().unwrap();

        for i in 0..vars.len() {
            assert!(solution.value(vars[i]) - ((i != 6) as i32 as f64) < 10e-6);
        }
    }

    #[test]
    fn test_example_abs() {
        let mut problem = ProblemVariables::new();
        let x = problem.add(variable().max(2));
        let t = problem.add(variable());
        let solution = problem.minimise(t).using(IntegerSolver::new)
            .with(constraint!(x-3 <= t))
            .with(constraint!(-(x-3) <= t))
            .solve().unwrap();

        assert!(solution.value(x) - 2. < 10e-6);
    }

    #[test]
    fn test_example_equal_split() {
        let mut problem = ProblemVariables::new();
        let x1 = problem.add(variable().integer().min(0).max(1));
        let x2 = problem.add(variable().integer().min(0).max(1));
        let t = problem.add(variable());
        let solution = problem.minimise(t).using(IntegerSolver::new)
            .with(constraint!((2*x1-1) + (2*x2-1) <= t))
            .with(constraint!(-((2*x1-1) + (2*x2-1)) <= t))
            .solve().unwrap();

        assert!(
            (solution.value(x1) - 1. < 10e-6
            && solution.value(x2) - 0. < 10e-6)
            || (solution.value(x1) - 0. < 10e-6
            && solution.value(x2) - 1. < 10e-6)
        );
    }

    #[test]
    fn test_example_equal_split2() {
        let weights_obj = [1., 0.4, 0.4];

        let mut problem = ProblemVariables::new();

        let vars: Vec<Variable> = (0..weights_obj.len())
            .map(|_| problem.add(variable().integer().min(0).max(1)))
            .collect();
        let diff = problem.add(variable());

        let constr_weights: Expression = weights_obj.iter()
            .zip(&vars)
            .map(|(w, v)| (2 * *v - 1) * *w)
            .sum();

        let solution = problem.minimise(diff).using(IntegerSolver::new)
            .with(constraint!(constr_weights.clone() <= diff))
            .with(constraint!(-constr_weights <= diff))
            .solve().unwrap();

        assert!(
            vars.len() == vars.iter().enumerate()
                .map(|(i, v)| solution.value(*v) - ((i == 0) as i32 as f64))
                .map(|d| (d < 10e-6) as usize)
                .sum()
            ||
            vars.len() == vars.iter().enumerate()
                .map(|(i, v)| solution.value(*v) - ((i != 0) as i32 as f64))
                .map(|d| (d < 10e-6) as usize)
                .sum()
        );
    }
}
