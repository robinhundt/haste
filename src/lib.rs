mod bencher;

pub use crate::bencher::Bencher;
pub use haste_derive::haste;

pub fn main() {
    let mut bencher = Bencher::new();
    for bench in __private::BENCHMARKS {
        bench(&mut bencher);
    }
}

#[doc(hidden)]
pub mod __private {
    use crate::Bencher;

    pub use linkme;
    pub use linkme::distributed_slice;

    #[distributed_slice]
    pub static BENCHMARKS: [fn(&mut Bencher)];
}
