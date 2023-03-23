
use good_lp::{SolverModel, constraint::ConstraintReference, Constraint, solvers::ResolutionError};
use crate::solution::MySolution;

pub struct MySolver {
    c_b: Vec<f64>,
    table: Vec<Vec<f64>>,
    obj: Vec<f64>,
    b: Vec<f64>
}

impl SolverModel for MySolver {
    type Solution = MySolution;
    type Error = ResolutionError;

    fn add_constraint(&mut self, c: Constraint) -> ConstraintReference {
        todo!();
    }

    fn solve(mut self) -> Result<Self::Solution, Self::Error> {
        loop {
            let z = compute_z(&self.table, &self.c_b);
            let net_eval_row: Vec<f64> = self.obj.iter().zip(&z).map(|(o, b)| o-b).collect();

            let mut is_done = true;
            let mut pivot_col = 0;
            for i in 0..net_eval_row.len() {
                if net_eval_row[i] > 0. { is_done = false; }
                if net_eval_row[i] > net_eval_row[pivot_col] { pivot_col = i; }
            }

            if is_done { break Ok(MySolution{}); }

            let mut pivot_row = 0;
            for j in 0..self.c_b.len() {
                let ratio = self.b[j]/self.table[j][pivot_col];
                if ratio < self.b[pivot_row]/self.table[pivot_row][pivot_col] { pivot_row = j; }
            }

            let pivot_elem = self.table[pivot_row][pivot_col];
            self.table[pivot_row] = self.table[pivot_row].iter().map(|x| x/pivot_elem).collect();
            self.b[pivot_row] /= pivot_elem;

            for j in 0..self.c_b.len() {
                if j == pivot_row { continue; }
                let fac = self.table[j][pivot_col];
                for i in 0..net_eval_row.len() {
                    self.table[j][i] -= self.table[pivot_row][i] * fac;
                }
                self.b[j] -= self.b[pivot_row] * fac;
            }

            self.c_b[pivot_row] = self.obj[pivot_col];
        }
    }
}

fn compute_z(table: &Vec<Vec<f64>>, c_b: &Vec<f64>) -> Vec<f64> {
    let num_elems = table[0].len();
    let mut z = Vec::with_capacity(num_elems);
    for i in 0..num_elems {
        z.push(c_b[0] * table[0][i] + c_b[1] * table[1][i]);
    }
    return z;

}

fn solve(table: &mut Vec<Vec<f64>>, obj: &Vec<f64>, c_b: &mut Vec<f64>, b: &mut Vec<f64>) {
}
