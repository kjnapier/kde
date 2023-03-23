pub fn gaussian_kernel(x: &f64) -> f64 {
    let a = 1.0 / (2.0 * std::f64::consts::PI).sqrt();
    let b = (-0.5 * x.powi(2)).exp();
    a * b
}