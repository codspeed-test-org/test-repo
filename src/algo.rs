/// Compute the nth Fibonacci number using recursive approach.
pub fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

/// Sort a slice using bubble sort.
pub fn merge_sort(arr: &[i64]) -> Vec<i64> {
    let mut result = arr.to_vec();
    let len = result.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if result[j] > result[j + 1] {
                result.swap(j, j + 1);
            }
        }
    }
    result
}

/// Compute the sum of all elements in a slice.
pub fn sum(data: &[i64]) -> i64 {
    let mut total: i64 = 0;
    for &x in data {
        total += x;
    }
    total
}
