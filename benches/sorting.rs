use codspeed_guide_rust::sorting::bubble_sort;

fn main() {
    divan::main();
}

// Generate synthetic test data
fn generate_random_vec(size: usize) -> Vec<i32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    (0..size)
        .map(|i| {
            let mut hasher = DefaultHasher::new();
            i.hash(&mut hasher);
            (hasher.finish() % 10000) as i32
        })
        .collect()
}

#[divan::bench(args = [100, 1000, 10_000])]
fn bench_bubble_sort(bencher: divan::Bencher, size: usize) {
    bencher
        .with_inputs(|| generate_random_vec(size))
        .bench_values(|data| bubble_sort(data));
}