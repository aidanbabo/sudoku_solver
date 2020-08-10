use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use sudoku::{ONLINE, JON};
use sudoku::solvers::sorted::solve;

fn sorted_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison");
    for (table_name, table) in &[("online", ONLINE), ("jon", JON)] {
        group.bench_with_input(BenchmarkId::new("sorted", table_name), table, |b, table| b.iter(|| solve(&mut table.clone())));
    }
}

criterion_group!(benches, sorted_benchmark);
criterion_main!(benches);
