fn main() {
    divan::main();
}

fn fib(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fib(n - 2) + fib(n - 1)
    }
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