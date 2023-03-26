use std::{error::Error, collections::{HashSet, HashMap}};
use good_lp::{constraint, solvers::highs::{highs, HighsSolution}, 
    ProblemVariables, variable, variable::{UnsolvedProblem}, SolverModel, 
    Solution, Constraint, Expression, Variable
};
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


struct IntegerSolver {
    problem: UnsolvedProblem,
    constraints: Vec<Constraint>,
    int_vars: HashSet<Variable>,
    best_solution: Option<HighsSolution>,
}

impl IntegerSolver {
    pub fn new() -> Self {
        let mut problem = ProblemVariables::new();
        let a = problem.add(variable().min(0));
        let b = problem.add(variable().min(0));
        let objective = 7*a + 6*b;
        let problem = problem.minimise(objective);
        
        let constr = vec![
            constraint!(2*a + 4*b <= 16),
            constraint!(3*a + 2*b <= 12),
        ];

        let mut int_vars = HashSet::new();
        int_vars.insert(a);
        int_vars.insert(b);

        IntegerSolver { 
            problem,
            constraints: constr,
            int_vars,
            best_solution: None,
        }
    }

    fn solve(self) -> Option<HighsSolution> {
        self.solve_rec(self.constraints.clone());

        return self.best_solution
    }

    fn solve_rec(&self, mut constraints: Vec<Constraint>) {
        let objective = self.problem.objective.clone();
        let variables = self.problem.variables.iter_variables_with_def();

        let mut solver = self.problem.clone().using(highs);
        for c in &constraints {
            solver = solver.with(c.clone());
        }
        let solution = solver.solve().unwrap();

        let objective_value = solution.eval(&objective);

        let next_var = variables
            .map(|(v, _)| v)
            .filter(|v| self.int_vars.contains(v))
            .filter(|v| solution.value(*v).fract() != 0.)
            .reduce(|v1, v2| if solution.value(v1) > solution.value(v2) { v1 } else { v2 });

        // TODO: Fix condition flow
        if let Some(best_solution) = &self.best_solution {
            if objective_value < objective.eval_with(best_solution) { return; }
        } 

        let Some(next) = next_var else { return; };

        let mut constraints_lower = constraints.clone();
        constraints_lower.push(constraint!(next <= solution.value(next).floor()));
       
        constraints.push(constraint!(next >= solution.value(next).ceil()));

        self.solve_rec(constraints_lower);
        self.solve_rec(constraints);
    }
}


fn main() {

    solve_true().unwrap();
}
