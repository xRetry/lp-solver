use std::collections::HashMap;
use good_lp::{Solution, Variable};


pub struct MySolution {
    pub(crate) variable_values: HashMap<Variable, f64>,
}

impl Solution for MySolution {
    fn value(&self, variable: Variable) -> f64 {
        *self.variable_values.get(&variable).unwrap()
    }
}
