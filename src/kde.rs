use crate::kernels::gaussian_kernel;
use crate::bandwidths::bandwidth_function;

pub struct KernelDensityEstimatorParameters {
    bandwidth_function: fn(&Vec<f64>) -> f64,
    kernel_function: fn(&f64) -> f64,
}

impl KernelDensityEstimatorParameters {
    pub fn new(bandwidth: fn(&Vec<f64>) -> f64, kernel: fn(&f64) -> f64) -> Self {
        Self { bandwidth_function: bandwidth, kernel_function: kernel }
    }

    pub fn default() -> Self {
        Self { bandwidth_function: bandwidth_function, kernel_function: gaussian_kernel }
    }
}

pub struct KernelDensityEstimator {
    data: Vec<f64>,
    bandwidth: f64,
    kernel: fn(&f64) -> f64,
}


impl KernelDensityEstimator {
    pub fn new(data: Vec<f64>, definition: KernelDensityEstimatorParameters) -> Self {
        let bandwidth = (definition.bandwidth_function)(&data);
        Self { data: data, bandwidth: bandwidth, kernel: definition.kernel_function }
    }

    pub fn gaussian(data: Vec<f64>) -> Self {
        let bandwidth = bandwidth_function(&data);
        Self { data: data, bandwidth: bandwidth, kernel: gaussian_kernel }
    }

    pub fn sample(&self, x: &f64) -> f64 {
        let mut sum = 0.0;
        for y in &self.data {
            sum += (self.kernel)(&((x - y) / self.bandwidth));
        }
        sum / (self.data.len() as f64 * self.bandwidth)
    }

}
