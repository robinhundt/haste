use haste::haste;

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

#[haste(args = [10, 32])]
fn bench_fib2(arg: u64) {
    fibonacci(arg);
}
