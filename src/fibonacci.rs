pub fn fib(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    let (mut a, mut b) = (1u64, 1u64);
    for _ in 2..=n {
        (a, b) = (b, a + b);
    }
    b
}

// Example of a dynamic benchmark
pub fn generate_large_json(size: usize) -> String {
    let items: Vec<_> = (0..size)
        .map(|i| format!(r#"{{"id":{},"name":"item_{}","value":{}}}"#, i, i, i * 10))
        .collect();
    format!("[{}]", items.join(","))
}