use clap::Parser;
use regex_lite::Regex;

#[derive(Parser)]
pub(crate) struct Cli {
    /// Filter your benchmarks with the provided regex
    pub(crate) filter: Option<Regex>,
    #[clap(long, hide(true))]
    bench: bool,
}
