use std::{
    fmt::{Display, Write},
    time::Duration,
};

use crate::{Label, sample::Sample, throughput::Throughput};

#[derive(Default)]
pub struct Results {
    results: Vec<BenchResult>,
}

impl Results {
    pub(crate) fn push(&mut self, value: BenchResult) {
        self.results.push(value);
    }
}

#[derive(Debug)]
pub(crate) struct BenchResult {
    pub(crate) label: Label,
    pub(crate) min_ns: f64,
    pub(crate) max_ns: f64,
    pub(crate) mean_ns: f64,
    pub(crate) throughput: Option<Throughput>,
}

impl BenchResult {
    pub(crate) fn from_samples(label: Label, samples: &[Sample]) -> BenchResult {
        let [min, max, sum] =
            samples
                .iter()
                .fold([f64::MAX, 0.0, 0.0], |[min, max, mut sum], sample| {
                    let min = min.min(sample.iter_time_ns);
                    let max = max.max(sample.iter_time_ns);
                    sum += sample.iter_time_ns;
                    [min, max, sum]
                });

        let mean = sum / samples.len() as f64;

        BenchResult {
            label: label,
            min_ns: min,
            max_ns: max,
            mean_ns: mean,
            throughput: None,
        }
    }

    pub(crate) fn with_throughput(self, throughput: Throughput) -> BenchResult {
        Self {
            throughput: Some(throughput),
            ..self
        }
    }
}

impl Display for BenchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min = scale_nanos(self.min_ns);
        let max = scale_nanos(self.max_ns);
        let mean = scale_nanos(self.mean_ns);
        write!(
            f,
            "{}: Min: {:.2} | Mean: {:.2} | Max: {:.2}",
            self.label, min, mean, max
        )?;
        if let Some(throughput) = self.throughput {
            let min = scale_throughput(self.min_ns, throughput);
            let max = scale_throughput(self.max_ns, throughput);
            let mean = scale_throughput(self.mean_ns, throughput);
            f.write_char('\n')?;
            write!(
                f,
                "Throughput: Min: {:.2} | Mean: {:.2}| Max: {:.2}",
                min, mean, max
            )?;
        }
        Ok(())
    }
}

pub(crate) struct Scaled {
    val: f64,
    unit: &'static str,
}

impl Display for Scaled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.val.fmt(f)?;
        f.write_char(' ')?;
        f.write_str(self.unit)?;
        Ok(())
    }
}

pub(crate) fn scale_nanos(ns: f64) -> Scaled {
    let (factor, unit) = if ns < 10_f64.powi(0) {
        (10_f64.powi(3), "ps")
    } else if ns < 10_f64.powi(3) {
        (10_f64.powi(0), "ns")
    } else if ns < 10_f64.powi(6) {
        (10_f64.powi(-3), "µs")
    } else if ns < 10_f64.powi(9) {
        (10_f64.powi(-6), "ms")
    } else {
        (10_f64.powi(-9), "s")
    };

    Scaled {
        val: ns * factor,
        unit,
    }
}

fn scale_throughput(ns: f64, throughput: Throughput) -> Scaled {
    let secs = Duration::from_nanos(ns as u64).as_secs_f64();

    match throughput {
        Throughput::Bytes(bytes) => {
            let bytes_per_s = bytes as f64 / secs;
            let (denom, unit) = if bytes_per_s < 1024.0_f64.powi(1) {
                (1024.0_f64.powi(0), "B/s")
            } else if bytes_per_s < 1024.0_f64.powi(2) {
                (1024.0_f64.powi(1), "KiB/s")
            } else if bytes_per_s < 1024.0_f64.powi(3) {
                (1024.0_f64.powi(2), "MiB/s")
            } else {
                (1024.0_f64.powi(3), "GiB/s")
            };
            Scaled {
                val: bytes_per_s / denom,
                unit,
            }
        }
        Throughput::Items(items) => {
            let items_per_s = items as f64 / secs;
            let (denom, unit) = if items_per_s < 1000.0_f64.powi(1) {
                (1000.0_f64.powi(0), "elem/s")
            } else if items_per_s < 1000.0_f64.powi(2) {
                (1000.0_f64.powi(1), "Kelem/s")
            } else if items_per_s < 1000.0_f64.powi(3) {
                (1000.0_f64.powi(2), "Melem/s")
            } else {
                (1000.0_f64.powi(3), "Gelem/s")
            };
            Scaled {
                val: items_per_s / denom,
                unit,
            }
        }
    }
}
