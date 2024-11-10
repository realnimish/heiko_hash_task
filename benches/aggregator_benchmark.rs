use criterion::{black_box, criterion_group, criterion_main, Criterion};
use heiko_hash_task::{aggregator, helpers::generate_random_hashes};

fn benchmark_aggregate_hashes(c: &mut Criterion) {
    let hashes = generate_random_hashes();

    c.bench_function("aggreagete hashes", |b| {
        b.iter(|| aggregator::aggregate_hashes(black_box(&hashes)))
    });

    c.bench_function("aggreagete hashes by parts", |b| {
        b.iter(|| aggregator::aggregate_hashes_by_parts(black_box(&hashes)))
    });

    c.bench_function("aggreagete hashes by parts (parallel-process)", |b| {
        b.iter(|| aggregator::parallel_aggregate_hashes_by_parts(black_box(hashes.clone())))
    });
}

criterion_group!(benches, benchmark_aggregate_hashes,);
criterion_main!(benches);
