/// A work-in-progress benchmarking library.
mod bench_result;
mod bencher;
mod cli;
mod config;
#[cfg(all(doctest, feature = "tokio"))]
mod doctests;
mod label;
mod sample;
mod sampling_mode;
pub mod throughput;

use crate::bench_result::Results;
pub use crate::bencher::Haste;
use crate::config::Config;
pub use crate::label::Label;
use clap::Parser;
pub use haste_macros::bench;
pub use throughput::Throughput;

/// A main function to call in your benchmark's main.
///
/// Call this function in your `benches/` benchmark main function. This will parse provided
/// arguments, apply filters, and run the corresponding benchmarks. The available CLI options
/// can be viewed with `cargo bench -- --help`  (note `--` after bench).
pub fn main() {
    let cli = cli::Cli::parse();
    let mut results = Results::default();
    let config = Config {
        filter: cli.filter,
        ..Default::default()
    };

    for bench in __private::BENCHMARKS {
        let mut haste = Haste::new(&mut results);
        haste.set_config(config.clone());
        bench(haste);
    }
}

#[doc(hidden)]
pub mod __private {
    use crate::Haste;

    pub use linkme;
    pub use linkme::distributed_slice;
    #[cfg(feature = "tokio")]
    pub use tokio;

    #[distributed_slice]
    pub static BENCHMARKS: [fn(Haste)];
}
