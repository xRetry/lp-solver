use minilp::{Problem, OptimizationDirection, ComparisonOp};

fn compute_z(table: &Vec<Vec<f64>>, c_b: &Vec<f64>) -> Vec<f64> {
    let num_elems = table[0].len();
    let mut z = Vec::with_capacity(num_elems);
    for i in 0..num_elems {
        z.push(c_b[0] * table[0][i] + c_b[1] * table[1][i]);
    }
    return z;

}

fn solve(table: &mut Vec<Vec<f64>>, obj: &Vec<f64>, c_b: &mut Vec<f64>, b: &mut Vec<f64>) {
    loop {
        let z = compute_z(&table, &c_b);
        let net_eval_row: Vec<f64> = obj.iter().zip(&z).map(|(o, b)| o-b).collect();

        let mut is_done = true;
        let mut pivot_col = 0;
        for i in 0..net_eval_row.len() {
            if net_eval_row[i] > 0. { is_done = false; }
            if net_eval_row[i] > net_eval_row[pivot_col] { pivot_col = i; }
        }

        if is_done { break; }

        let mut pivot_row = 0;
        for j in 0..c_b.len() {
            let ratio = b[j]/table[j][pivot_col];
            if ratio < b[pivot_row]/table[pivot_row][pivot_col] { pivot_row = j; }
        }

        let pivot_elem = table[pivot_row][pivot_col];
        table[pivot_row] = table[pivot_row].iter().map(|x| x/pivot_elem).collect();
        b[pivot_row] /= pivot_elem;

        for j in 0..c_b.len() {
            if j == pivot_row { continue; }
            let fac = table[j][pivot_col];
            for i in 0..net_eval_row.len() {
                table[j][i] -= table[pivot_row][i] * fac;
            }
            b[j] -= b[pivot_row] * fac;
        }

        c_b[pivot_row] = obj[pivot_col];
    }
}

fn main() {

    let obj = vec![7., 6., 0., 0.];
    let mut table = vec![
        vec![2., 4., 1., 0.],
        vec![3., 2., 0., 1.],
    ];
    let mut c_b = vec![0., 0.];
    let mut b = vec![16., 12.];

    solve(&mut table, &obj, &mut c_b, &mut b);

    println!("{:?}", b);

}
