use good_lp::{Solution, Variable, solvers::highs::HighsSolution};

pub struct CustomSolution {
    pub num_evals: usize,
    values: Vec<f64>,
}

impl CustomSolution {
    pub fn new(values: Vec<f64>) -> Self {
        CustomSolution{
            values,
            num_evals: 0,
        }

    }

    pub fn from(solution: &HighsSolution, num_vars: usize, num_evals: usize) -> Self {
        let values = (0..num_vars)
            .map(|i| Variable::new(i))
            .map(|v| solution.value(v))
            .collect();

        CustomSolution{
            values,
            num_evals,
        }
    }
}

impl Solution for CustomSolution {
    fn value(&self, variable: Variable) -> f64 {
        self.values[variable.index()]
    }
}
