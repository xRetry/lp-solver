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

pub fn linear_distribution(num_weights: usize, min_value: f64, max_value: f64) -> Vec<f64> {
    let delta = (max_value - min_value) / num_weights as f64;
    (0..num_weights)
        .map(|i| min_value + i as f64 * delta)
        .collect()
}
