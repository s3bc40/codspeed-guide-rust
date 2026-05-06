pub fn fib(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fib(n - 2) + fib(n - 1)
    }
}

// Example of a dynamic benchmark
pub fn generate_large_json(size: usize) -> String {
    let items: Vec<_> = (0..size)
        .map(|i| format!(r#"{{"id":{},"name":"item_{}","value":{}}}"#, i, i, i * 10))
        .collect();
    format!("[{}]", items.join(","))
}