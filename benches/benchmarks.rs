use codspeed_bench_demo::algo;

fn main() {
    divan::main();
}

#[divan::bench(args = [1, 10, 20, 30])]
fn fibonacci(n: u64) -> u64 {
    algo::fibonacci(n)
}

#[divan::bench(args = [100, 1000, 10000])]
fn merge_sort(n: usize) -> Vec<i64> {
    let data: Vec<i64> = (0..n as i64).rev().collect();
    algo::merge_sort(&data)
}

#[divan::bench(args = [1000, 10000, 100000])]
fn sum(n: usize) -> i64 {
    let data: Vec<i64> = (0..n as i64).collect();
    algo::sum(&data)
}
