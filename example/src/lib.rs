use std::{hint::black_box, thread, time::Duration};

use haste::{Haste, throughput::Throughput};
use tokio::runtime::Runtime;

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[haste::bench(args = [2, 5])]
fn bench_fib(arg: u64) {
    fibonacci(arg);
}

#[haste::bench(args = [10, 100, 1000, 10000])]
fn bench_add(arg: u64) {
    add(arg, arg);
}

#[haste::bench]
fn use_haste_directly(mut haste: Haste) {
    haste
        .with_throughput(Throughput::Bytes(100))
        .bench("fib", || {
            fibonacci(black_box(20));
        });
}

#[haste::bench]
fn bench_async(mut haste: Haste) {
    let rt = Runtime::new().unwrap();
    haste.bench_async("bench async sleep", &rt, async || {
        tokio::time::sleep(Duration::from_millis(50)).await
    });
}

#[haste::bench]
async fn bench_async_overhead() {
    add(5, 5);
}

#[haste::bench(args = [1,2,3], throughput = Throughput::Bytes(arg as usize * 100))]
fn bench_throughput(arg: u64) {
    thread::sleep(Duration::from_millis(arg));
}
