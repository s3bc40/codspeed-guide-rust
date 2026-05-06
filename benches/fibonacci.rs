use tokio::time::Duration;
use tokio::{runtime::Runtime, time::sleep};
use codspeed_guide_rust::fibonacci::{fib, generate_large_json};

fn main() {
    divan::main();
}

// Preventing dead code elimination -> black_box ("Pretend you don't know what this value is.")
#[divan::bench]
fn bench_fib() -> u64 {
    fib(divan::black_box(10))
}

// O(2^n) complexity example
// Note: O(1) constants, O(n) linear, O(n^2) quadratic, O(2^n) exponential
#[divan::bench(args = [1, 5, 15, 25, 35])]
fn bench_fib_args(n: u64) -> u64 {
    fib(divan::black_box(n))
}

// Benchmarking only what we care about
#[divan::bench(args = [100, 1_000, 10_000])]
fn search_vec(bencher: divan::Bencher, size: usize) {
    let vec: Vec<i32> = (0..size as i32).collect();
    let target = size as i32 / 2;

    bencher.bench_local(|| {
        vec.iter().find(|x| **x == target)
        // or vec.iter().find(|&&x| x == target)
    });
}

// Bench same operation with different types
// conversion from &str to both &str (no-op) and String (allocation)
#[divan::bench(types = [&str, String])]
fn bench_from_str<'a, T>() -> T
where T: From<&'a str>,
{
    divan::black_box("hello world").into()
}

#[divan::bench(args = [10, 100, 1_000])]
fn bench_parse_json(bencher: divan::Bencher, size: usize) {
    bencher.with_inputs(|| {
        // each iteration with new test JSON and not measured
        generate_large_json(size)
    }).bench_values(|json_string| {
        // benchmarking the actual parsing
        serde_json::from_str::<serde_json::Value>(&json_string)
    });
}

// Async benchmarking example
#[divan::bench]
fn async_sleep_bench(bencher: divan::Bencher) {
    let rt = Runtime::new().unwrap();

    bencher.bench_local(|| {
        rt.block_on(async {
            // Simulate an async operation
            sleep(Duration::from_millis(100)).await;
        })
    });
}