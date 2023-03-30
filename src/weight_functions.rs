use rand::Rng;

pub fn equal_distribution(num_weigths: usize, value: f64) -> Vec<f64> {
    vec![value; num_weigths]
}

pub fn random_distribution(num_weigths: usize, min_value: f64, max_value: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..num_weigths)
        .map(|_| rng.gen_range(min_value..max_value))
        .collect()
}
