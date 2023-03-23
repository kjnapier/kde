use rand::Rng;
use rand_distr::{Distribution, Normal};

use plotly::common::Mode;
use plotly::{Plot, Scatter};

fn bandwidth(v: &Vec<f64>) -> f64 {
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    let std_dev = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64).sqrt();
    let n = v.len() as f64;
    let h = 1.06 * std_dev * n.powf(-0.2);
    h
}

fn kernel(x: f64) -> f64 {
    let a = 1.0 / (2.0 * std::f64::consts::PI).sqrt();
    let b = (-0.5 * x.powi(2)).exp();
    a * b
}

fn kernel_density_estimation(x: f64, v: &Vec<f64>, h: &f64) -> f64 {
    let mut sum = 0.0;
    for y in v {
        sum += kernel((x - y) / h);
    }
    sum / (v.len() as f64 * h)
}

use std::time::Instant;

fn main() {
    // generate a normal distribution with mean 0 and standard deviation 1
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut rng = rand::thread_rng();

    //  sample 10000 points from the distribution
    let v: Vec<f64> = (0..1_000_000)
        .map(|_| normal.sample(&mut rng))
        .collect();

    // compute the mean and standard deviation
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    let std_dev = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64).sqrt();

    println!("mean: {}, std_dev: {}", mean, std_dev);

    // compute the bandwidth
    let h = bandwidth(&v);
    println!("bandwidth: {}", h);

    // compute the kernel density estimation
    let x = 0.0;

    let now = Instant::now();
    let y = kernel_density_estimation(x, &v, &h);
    println!("kernel density estimation: {}", y);
    println!("time: {} ms", now.elapsed().as_millis());

    // // put the kde on a spline
    // let mut spline = Spline::new();
    // spline.set_points(&v, &v);
    // let y = spline.interpolate(x);
    // println!("spline: {}", y);

    let x: Vec<f64> = (0..100).map(|x| x as f64 / 10.0 - 5.0).collect();

    let now = Instant::now();
    let mut y = Vec::with_capacity(x.len());
    let y: Vec<f64> = x.iter().map(|x| kernel_density_estimation(*x, &v, &h)).collect();
    println!("time: {} ms", now.elapsed().as_millis());

    let trace = Scatter::new(x, y)
        .name("kernel density estimation")
        .mode(Mode::Lines);
    let mut fg = Plot::new();
    fg.add_trace(trace);
    fg.show();
}
