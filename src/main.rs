fn random() -> impl Iterator<Item=u32> {
    let mut r = 92;
    std::iter::repeat_with(move || {
        r ^= r << 13;
        r ^= r >> 17;
        r ^= r << 5;
        r
    })
}

fn main() {
    const N: usize = 100_000_000;
    const N_IDX: usize = N; // change to N + 1 to see that bounds checks are **not** elided.
    let indexes: Vec<usize> = (0..N_IDX).collect(); 
    let xs: Vec<u32> = random().take(N).collect();

    let r1 = run_benchmark("sum_indirectly", || sum_indirectly(&xs, &indexes));
    let r2 = run_benchmark("sum_indirectly_unchecked", || sum_indirectly_unchecked(&xs, &indexes));

    let r3 = run_benchmark("sum", || sum(&xs, 0, N_IDX));
    let r4 = run_benchmark("sum_unchecked", || sum_unchecked(&xs, 0, N_IDX));

    let r5 = run_benchmark("sum_hoisted_checks", || sum_hoisted_checks(&xs, 0, N_IDX));
    let r6 = run_benchmark("sum_iter_sum", || sum_iter_sum(&xs, 0, N_IDX));

    assert!(r1 == r2 && r2 == r3 && r3 == r4 && r4 == r5 && r5 == r6);
}

#[inline(never)]
fn run_benchmark<F: Fn() -> T, T>(name: &str, f: F) -> Vec<T> {
    println!("{}:", name);
    let n = 300;
    let mut res = Vec::with_capacity(n);
    let mut times = Vec::with_capacity(n);
    for _ in 0..n {
        let start = std::time::Instant::now();
        res.push(f());
        times.push(start.elapsed());
    }
    println!("{:?}", times.into_iter().min().unwrap());
    println!("\n");
    res
}


#[inline(never)]
fn sum_indirectly(xs: &[u32], indexes: &[usize]) -> u32 {
    let mut sum = 0u32;
    for &idx in indexes {
        let x = xs[idx];
        sum = sum.wrapping_add(x);
    }
    sum
}

#[inline(never)]
fn sum_indirectly_unchecked(xs: &[u32], indexes: &[usize]) -> u32 {
    let mut sum = 0u32;
    for &idx in indexes {
        let x = unsafe { *xs.get_unchecked(idx) };
        sum = sum.wrapping_add(x);
    }
    sum
}

#[inline(never)]
fn sum(xs: &[u32], lo: usize, hi: usize) -> u32 {
    let mut sum = 0u32;
    for idx in lo..hi {
        let x = xs[idx];
        sum = sum.wrapping_add(x);
    }
    sum
}

#[inline(never)]
fn sum_unchecked(xs: &[u32], lo: usize, hi: usize) -> u32 {
    let mut sum = 0u32;
    for idx in lo..hi {
        let x = unsafe { *xs.get_unchecked(idx) };
        sum = sum.wrapping_add(x);
    }
    sum
}

#[inline(never)]
fn sum_hoisted_checks(xs: &[u32], lo: usize, hi: usize) -> u32 {
    let mut sum = 0u32;
    for &x in &xs[lo..hi] {
        sum = sum.wrapping_add(x);
    }
    sum
}


#[inline(never)]
fn sum_iter_sum(xs: &[u32], lo: usize, hi: usize) -> u32 {
    xs[lo..hi].iter()
        .map(|&x| std::num::Wrapping(x))
        .sum::<std::num::Wrapping<u32>>()
        .0
}

