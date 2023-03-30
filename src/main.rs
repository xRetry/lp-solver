use csv;
use std::fs::OpenOptions;
use lp_solver::{comparison::{SolutionSummary, compare_solvers}, 
    weight_functions::random_distribution
};
use chrono::Local;
use itertools::Itertools;

fn write_to_file(solution: SolutionSummary, file_name: &str) {

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)
        .unwrap();

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    wtr.serialize(solution).unwrap();

    wtr.flush().unwrap();
}

fn main() {
    let num_vars = [3, 5, 10];
    let max_values = [10., 100.];
    // TODO: Add max_values to SolutionSummary

    let time_str = Local::now().format("%Y%m%d_%H%M%S");
    let file = format!("data/data_{}.csv", time_str);

    num_vars.into_iter()
        .cartesian_product(max_values)
        .map(|(n, m)| move || random_distribution(n, 0., m))
        .map(compare_solvers)
        .flatten()
        .for_each(|s| write_to_file(s, &file));
}
