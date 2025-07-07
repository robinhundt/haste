use std::{cmp, time::Duration};

use crate::config::Config;

#[derive(Debug)]
pub(crate) enum SamplingMode {
    Linear { sampling_factor: usize },
    Flat { sample_size: usize },
}

impl SamplingMode {
    pub(crate) fn decide_mode(config: &Config, warmup_time: Duration, warmup_iters: usize) -> Self {
        let warmup_time = warmup_time.as_nanos() as f64;
        let warmup_mean = warmup_time / warmup_iters as f64;
        let target_time = config.sample_time.as_nanos() as f64;

        // From criterion
        // Solve for d in: [d + 2*d + 3*d + ... + c.sample_count*d] * warmup_mean = c.sample_time
        // where d is the sampling_factor
        let sampling_factor = {
            let n = config.sample_count;
            let unscaled_total_iters = n * (n + 1) / 2;
            let d = target_time / warmup_mean / unscaled_total_iters as f64;
            cmp::max(d.ceil() as usize, 1)
        };

        if sampling_factor == 1 {
            // target_time = c.sample_count * sample_size * mean
            let sample_size = target_time / warmup_mean / config.sample_count as f64;
            Self::Flat {
                sample_size: sample_size.ceil() as usize,
            }
        } else {
            Self::Linear {
                sampling_factor: sampling_factor,
            }
        }
    }

    pub(crate) fn sample_sizes(&self, config: &Config) -> impl Iterator<Item = usize> {
        (1..=config.sample_count).map(move |iter| match self {
            SamplingMode::Linear { sampling_factor } => iter * sampling_factor,
            SamplingMode::Flat { sample_size } => *sample_size,
        })
    }
}
