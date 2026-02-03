use std::{
    hint::black_box,
    mem,
    time::{Duration, Instant},
};

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

        if let Some(regex) = &c.filter
            && !regex.is_match(&label.to_string())
        {
            return;
        }
        let (warmup_time, warmup_iters) = self.warmup(&mut func);
        let sampling_mode = SamplingMode::decide_mode(c, warmup_time, warmup_iters);
        let mut samples = Vec::with_capacity(c.sample_count);

        let bench_time_start = Instant::now();
        for sample_size in sampling_mode.sample_sizes(&self.config) {
            let mut returns: Vec<R> = Vec::with_capacity(sample_size);
            // pre-fault vec to reduce overhead of memory allocations during extend call
            pre_fault_vec(&mut returns);
            let sample_start = Instant::now();
            returns.extend((0..sample_size).map(|_| black_box(func())));
            let sample_duration = sample_start.elapsed();
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
                black_box(func());
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

fn pre_fault_vec<T>(v: &mut Vec<T>) {
    // pre-fault the vec by volatile writing zero bytes to its spare capacity
    // We assume a page size of 4 kib, while there are systems with larger page sizes
    // this only makes the pre-faulting slower by a negligible amount. It is very
    // unlikely that this code is run on systems with a page size smaller.
    const PAGE_SIZE: usize = 4096;
    let total_bytes = v.spare_capacity_mut().len() * mem::size_of::<T>();
    let total_bytes = isize::try_from(total_bytes).expect("Allocation can't be larger than isize");
    let ptr = v.spare_capacity_mut().as_mut_ptr().cast::<u8>();
    for offset in (0..total_bytes).step_by(PAGE_SIZE) {
        // Safety:
        // - offset is within bounds of the capacity
        // - 0_u8 is valid to write into a MaybeUninit<T> for any T as
        //   MaybeUninit drops all validity requirements
        unsafe { ptr.offset(offset).write_volatile(0) };
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZero;

    use crate::bencher::pre_fault_vec;

    // Should be executed under miri
    #[test]
    fn test_pre_fault() {
        let mut v: Vec<NonZero<u8>> = Vec::with_capacity(10000);
        pre_fault_vec(&mut v);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 10000);
    }
}
