#[macro_use]
extern crate criterion;
use criterion::{Criterion, Benchmark};

fn with_iter(data: &[u32]) -> u32 {
    let mut result = 0;
    for b in data.iter() {
        result += b;
    }
    result
}

fn with_index(data: &[u32]) -> u32 {
    let mut result = 0;
    for i in 0..data.len() {
        result += data[i];
    }
    result
}

fn just_sum(data: &[u32]) -> u32 {
    data.iter().sum()
}

fn bench(c: &mut Criterion) {
    let data = [1; 50000];
    c.bench("iteration",
            Benchmark::new("with iter", move |b| b.iter(|| with_iter(&data)))
                .with_function("with index", move |b| b.iter(|| with_index(&data)))
                .with_function("just sum", move |b| b.iter(|| just_sum(&data))));
}

criterion_group!(benches, bench);
criterion_main!(benches);
