use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use sudoku::{ONLINE, JON};
use sudoku::solvers::entries::solve;

fn entries_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison");
    for (table_name, table) in &[("online", ONLINE), ("jon", JON)] {
        group.bench_with_input(BenchmarkId::new("entries", table_name), table, |b, table| b.iter(|| solve(&mut table.clone())));
    }
}

criterion_group!(benches, entries_benchmark);
criterion_main!(benches);
