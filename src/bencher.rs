use std::time::{Duration, Instant};

pub struct Bencher {
    config: Config,
    results: Vec<BenchResult>,
}

struct Config {
    warmup: Duration,
    sample_time: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            warmup: Duration::from_millis(500),
            sample_time: Duration::from_secs(2),
        }
    }
}

#[derive(Debug)]
struct BenchResult {
    name: &'static str,
    time: Duration,
}

impl Bencher {
    pub fn new() -> Self {
        let config = Config::default();
        Self {
            config,
            results: vec![],
        }
    }

    pub fn bench_function<R, F: FnMut() -> R>(&mut self, name: &'static str, mut func: F) {
        let warmup_start = Instant::now();
        let mut warmup_iters = 0;
        while warmup_start.elapsed() < self.config.warmup {
            func();
            warmup_iters += 1;
        }

        let time_single_run = warmup_start.elapsed().div_f64(warmup_iters as f64);

        let runs_needed = self
            .config
            .sample_time
            .div_duration_f32(time_single_run)
            .ceil() as usize;

        let now = Instant::now();
        for _ in 0..runs_needed {
            func();
        }
        let time = now.elapsed().div_f64(runs_needed as f64);
        self.results.push(dbg!(BenchResult { name, time }));
    }
}
