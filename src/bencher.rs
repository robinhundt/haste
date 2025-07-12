use std::time::{Duration, Instant};

#[cfg(feature = "tokio")]
use tokio::runtime::Runtime;

use crate::{
    bench_result::{BenchResult, Results, scale_nanos},
    config::Config,
    label::Label,
    sample::Sample,
    sampling_mode::SamplingMode,
    throughput::Throughput,
};

pub struct Haste<'a> {
    pub(crate) config: Config,
    pub(crate) results: &'a mut Results,
}

impl<'a> Haste<'a> {
    pub fn new(results: &'a mut Results) -> Self {
        Self {
            config: Config::default(),
            results,
        }
    }
    pub fn with_warmup(&mut self, duration: Duration) -> &mut Self {
        self.config.warmup = duration;
        self
    }

    pub fn with_sample_time(&mut self, duration: Duration) -> &mut Self {
        self.config.sample_time = duration;
        self
    }

    pub fn with_sample_count(&mut self, sample_count: usize) -> &mut Self {
        self.config.sample_count = sample_count;
        self
    }

    pub fn with_throughput(&mut self, throughput: Throughput) -> &mut Self {
        self.config.throughput = Some(throughput);
        self
    }

    pub fn bench<R, F>(&mut self, label: impl Into<Label>, mut func: F)
    where
        F: FnMut() -> R,
    {
        let label = label.into();
        let c = &self.config;

        if let Some(regex) = &c.filter {
            if !regex.is_match(&label.to_string()) {
                return;
            }
        }
        let (warmup_time, warmup_iters) = self.warmup(&mut func);

        let sampling_mode = SamplingMode::decide_mode(c, warmup_time, warmup_iters);

        let mut samples = Vec::with_capacity(c.sample_count);

        let bench_time_start = Instant::now();
        for sample_size in sampling_mode.sample_sizes(&self.config) {
            let mut returns: Vec<R> = Vec::with_capacity(sample_size);
            let sample_start = Instant::now();
            // TODO how to better handle drop? This can introduce a lot of overhead,
            // expecially when we have e.g. a page fault when pushing...
            // Maybe we can write random data to the spare capacity beforehand or already
            // use a vec for the returns of the warmup and reuse it here
            returns.extend((0..sample_size).map(|_| func()));
            let sample_duration = sample_start.elapsed();
            unsafe {
                returns.set_len(sample_size);
            }
            let sample = Sample::from_duration(sample_duration, sample_size);
            samples.push(sample);
        }
        let bench_time = bench_time_start.elapsed();

        let mut res = BenchResult::from_samples(label, &samples);
        if let Some(throughput) = c.throughput {
            res = res.with_throughput(throughput);
        }
        eprintln!(
            "{res}\t\tTotal Time: {:.2}",
            scale_nanos(bench_time.as_nanos() as f64)
        );

        self.results.push(res);
    }

    #[cfg(feature = "tokio")]
    pub fn bench_async<R, F>(&mut self, label: impl Into<Label>, rt: &Runtime, mut func: F)
    where
        F: AsyncFnMut() -> R,
    {
        let func = || rt.block_on(func());
        self.bench(label, func);
    }

    pub(crate) fn warmup<F, R>(&self, func: &mut F) -> (Duration, usize)
    where
        F: FnMut() -> R,
    {
        let warmup_start = Instant::now();
        let mut warmup_iters = 0;
        let mut warmup_sample_size = 1;
        while warmup_start.elapsed() < self.config.warmup {
            for _ in 0..warmup_sample_size {
                func();
            }
            warmup_iters += warmup_sample_size;
            warmup_sample_size *= 2;
        }
        let warmup_time = warmup_start.elapsed();
        (warmup_time, warmup_iters)
    }

    pub(crate) fn set_config(&mut self, config: Config) {
        self.config = config;
    }
}
