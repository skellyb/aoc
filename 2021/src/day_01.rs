/// 1. How many values are greater than the previous value?
/// 2. How many sums of a 3-value sliding window are greater than the previous sum?
pub fn run(input: &str) -> (i32, i32) {
    let depths: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();

    // part one
    let mut prev = depths[0];
    let mut increasing = 0;
    for d in depths[1..].iter() {
        if *d > prev {
            increasing += 1;
        }
        prev = *d;
    }

    // part two
    let sums: Vec<i32> = depths.windows(3).map(|d| d.iter().sum()).collect();
    let mut prev = sums[0];
    let mut increasing_sums = 0;
    for s in sums[1..].iter() {
        if *s > prev {
            increasing_sums += 1;
        }
        prev = *s;
    }

    (increasing, increasing_sums)
}
