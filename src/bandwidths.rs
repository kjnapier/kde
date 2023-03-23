pub fn bandwidth_function(v: &Vec<f64>) -> f64 {
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    let std_dev = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64).sqrt();
    let n = v.len() as f64;
    let h = 1.06 * std_dev * n.powf(-0.2);
    h
}