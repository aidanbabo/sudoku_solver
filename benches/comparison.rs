use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use sudoku::solvers::{arrayvec, basic, entries, sorted};
use sudoku::{JON, ONLINE};

fn comparison_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison");
    for (table_name, table) in &[("online", ONLINE), ("jon", JON)] {
        group.bench_with_input(BenchmarkId::new("basic", table_name), table, |b, table| {
            b.iter(|| basic::solve(&mut table.clone()))
        });
        group.bench_with_input(BenchmarkId::new("sorted", table_name), table, |b, table| {
            b.iter(|| sorted::solve(&mut table.clone()))
        });
        group.bench_with_input(
            BenchmarkId::new("entries", table_name),
            table,
            |b, table| b.iter(|| entries::solve(&mut table.clone())),
        );
        group.bench_with_input(
            BenchmarkId::new("arrayvec", table_name),
            table,
            |b, table| b.iter(|| arrayvec::solve(&mut table.clone())),
        );
    }
}

criterion_group!(benches, comparison_benchmark);
criterion_main!(benches);
