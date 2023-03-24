fn find_pivot(table: &Vec<Vec<f64>>) -> Option<(usize, usize)> {
    let mut pivot_col = 0;
    let mut max_change = f64::MIN;
    for col in 1..table[0].len()-1 {
        let mut z = 0.;
        for row in 1..table.len() {
            z += table[row][0] * table[row][col];
        }
        let net_change = table[0][col] - z;

        if net_change > max_change { 
            max_change = net_change; 
            pivot_col = col;
        }
    }

    if max_change <= 0. { return None; }

    let mut pivot_row = 0;
    let mut min_ratio = f64::MAX;
    for row in 1..table.len() {
        let ratio = table[row].last().unwrap() / table[row][pivot_col];
        if ratio < min_ratio {
            min_ratio = ratio;
            pivot_row = row;
        }
    }

    Some((pivot_row, pivot_col))
}

fn pivot_table(table: &mut Vec<Vec<f64>>, pivot_row: usize, pivot_col: usize) {
    let pivot_elem = table[pivot_row][pivot_col];

    table[pivot_row][0] = table[0][pivot_col];

    for col in 1..table[pivot_row].len() {
        table[pivot_row][col] /= pivot_elem;
    }

    for row in 1..table.len() {
        if row == pivot_row { continue; }

        let fac = table[row][pivot_col];
        for col in 1..table[row].len() {
            table[row][col] -= table[pivot_row][col] * fac;
        }
    }
}

pub fn solve_simplex(mut table: Vec<Vec<f64>>, mut basic_var_cols: Vec<usize>) -> Vec<(i32, f64)> {
    loop {
        let Some((pivot_row, pivot_col)) = find_pivot(&mut table) else { break; };

        basic_var_cols[pivot_row-1] = pivot_col;
        
        pivot_table(&mut table, pivot_row, pivot_col);
    }

    let obj_value = (1..table.len())
        .map(|row| table[row][0] * table[row].last().unwrap())
        .sum();

    (0..table.len())
        .map(|row| {
            if row == 0 {
                (-1, obj_value)
            } else {
                (basic_var_cols[row-1] as i32, *table[row].last().unwrap())
            }
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::solve_simplex;

    #[test]
    fn example1() {
        let table = vec![
            vec![0., 7., 6., 0., 0., 0.],
            vec![0., 2., 4., 1., 0., 16.],
            vec![0., 3., 2., 0., 1., 12.],
        ];

        let sol = solve_simplex(table, vec![2, 3]);
        let tol = 10e-6;
        assert!(sol[0].1 - 32. < tol);
        assert!(sol[1].1 - 3. < tol);
        assert!(sol[2].1 - 2. < tol);
    }
}
