use std::hint::black_box;

use haste::haste;

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

#[haste]
fn bench_fib2() {
    fibonacci(black_box(32));
}
