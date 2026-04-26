use std::time::Instant;

fn main() {
    let start = Instant::now();
    let n = 30;
    let result = fib(n);
    let elapsed = start.elapsed();
    println!("fib({n}) = {result} in {elapsed:?}");
}

fn fib(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_base_cases() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn fib_known_values() {
        assert_eq!(fib(10), 55);
        assert_eq!(fib(20), 6765);
        assert_eq!(fib(30), 832_040);
    }
}
