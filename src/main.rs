use rand;
use rand_distr::{Distribution, Normal};
use std::time::Instant;
use rayon::prelude::*;

use plotly::common::Mode;
use plotly::{Plot, Scatter};


use kde::KernelDensityEstimator;

fn main() {

    // generate a normal distribution with mean 0 and standard deviation 1
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut rng = rand::thread_rng();

    //  sample 10000 points from the distribution
    let v: Vec<f64> = (0..1_000_000)
        .map(|_| normal.sample(&mut rng))
        .collect();

    let kde = KernelDensityEstimator::gaussian(v);
    
    let x: Vec<f64> = (0..100).map(|x| x as f64 / 10.0 - 5.0).collect();
    let now = Instant::now();
    let y = x.par_iter().map(|x| kde.sample(&x)).collect::<Vec<f64>>();
    println!("time: {} ms", now.elapsed().as_millis());

    let trace = Scatter::new(x, y)
        .name("kernel density estimation")
        .mode(Mode::Lines);
    let mut fg = Plot::new();
    fg.add_trace(trace);
    fg.show();

}
