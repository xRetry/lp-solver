use csv;
use std::fs::OpenOptions;
use lp_solver::{comparison::{SolutionSummary, compare_solvers}, 
    weight_functions::{random_distribution, equal_distribution, linear_distribution},
};
use chrono::Local;
use std::time::Duration;

fn write_to_file(wtr: &mut csv::Writer<std::fs::File>, solution: SolutionSummary)  {

    wtr.serialize(solution).unwrap();
    wtr.flush().unwrap();
}

fn main() {
    let time_str = Local::now().format("%Y%m%d_%H%M%S");
    let file_name = format!("data/data_{}.csv", time_str);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)
        .unwrap();

    let add_header = false;
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(add_header)
        .from_writer(file);

    let mut num_vars = 5;
    let num_repeats = 25;
    let min_weight = 1.;
    let max_weight = 100.;

    loop {
        num_vars = num_vars + 2;
        for i in 0..num_repeats {
            print!("Vars: {}, Iter: {}/{} ... ", num_vars, i, num_repeats);
            //let weights_fn = || random_distribution(num_vars, min_weight, max_weight);
            let weights_fn = || linear_distribution(num_vars, min_weight, max_weight);
            //let weights_fn = || equal_distribution(num_vars, max_weight);
            let solutions = compare_solvers(weights_fn);

            let duration = solutions[1].duration;
            println!("{:?}", duration);

            solutions.into_iter()
                .for_each(|s| write_to_file(&mut wtr, s));

        }
    }
}
