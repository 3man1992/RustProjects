extern crate rand;
extern crate csv;

use rand::distributions::{Uniform, Distribution};
use rand::Rng;
use std::error::Error;
use std::fs::File;

struct ImhomogeneousProcess {
    kernel: String,
    t: f64,
    peak_time: f64,
    width: f64,
    peak_intensity: f64,
    events: Vec<f64>,
}

impl ImhomogeneousProcess {
    fn new(t: f64, peak_time: f64, width: f64, peak_intensity: f64, kernel: &str) -> Self {
        let kernel = kernel.to_string();
        let mut process = ImhomogeneousProcess {
            kernel,
            t,
            peak_time,
            width,
            peak_intensity,
            events: vec![],
        };
        
        process.events = process.simulate_gaussian_intensity();
        process
    }

    fn gaussian(&self, x: f64, mu: f64, sigma: f64) -> f64 {
        (-((x - mu).powi(2)) / (2.0 * sigma.powi(2))).exp()
    }

    fn cif_function_gaussian(&self, t: f64) -> f64 {
        self.gaussian(t, self.peak_time, self.width) * self.peak_intensity
    }

    fn sample_inhomogeneous_pp_thinning_v2(&self) -> Vec<f64> {
        let mut n = 0;
        let mut m = 0;
        let mut point = vec![0.0];
        let mut s = vec![0.0];
        let lambda_bar = (0..(self.t * 100.0) as usize)
            .map(|x| self.cif_function_gaussian(x as f64 * 0.01))
            .fold(0.0_f64, f64::max);

        while s[m] < self.t {
            let u = rand::thread_rng().gen::<f64>();
            let w = -u.ln() / lambda_bar;
            s.push(s[m] + w);
            let d = rand::thread_rng().gen::<f64>();

            if d <= self.cif_function_gaussian(s[m + 1]) / lambda_bar {
                point.push(s[m + 1]);
                n += 1;
            }

            m += 1;
        }

        if point.last().unwrap() <= &self.t {
            point[1..].to_vec()
        } else {
            point[1..(point.len() - 1)].to_vec()
        }
    }

    fn simulate_gaussian_intensity(&self) -> Vec<f64> {
        self.sample_inhomogeneous_pp_thinning_v2()
    }
}

// Add this function to save spike_times to a CSV file
fn save_spike_times_to_csv(spike_times: &[f64], output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(File::create(output_file)?);

    for time in spike_times {
        wtr.write_record(&[time.to_string()])?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() {
    let t = 10.0;
    let peak_time_of_kernel = 5.0;
    let width_of_kernel = 1.0;
    let peak_intensity_of_kernel = 50.0;
    let kernel = "gaussian";

    let object = ImhomogeneousProcess::new(
        t,
        peak_time_of_kernel,
        width_of_kernel,
        peak_intensity_of_kernel,
        kernel,
    );

    let spike_times = object.events;
    println!("{:?}", spike_times);

    // Save spike_times to a CSV file
    match save_spike_times_to_csv(&spike_times, "spike_times.csv") {
        Ok(_) => println!("Spike times saved to 'spike_times.csv'."),
        Err(err) => eprintln!("Error saving spike times to CSV: {}", err),
    }
}

