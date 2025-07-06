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

pub fn main() {
    let cli = cli::Cli::parse();
    let mut results = Results::default();
    let mut config = Config::default();
    config.filter = cli.filter;

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
