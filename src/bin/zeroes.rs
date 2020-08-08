#![feature(test)]
extern crate test;
use test::Bencher;

use sudoku::solvers::{basic, entries, sorted};
use sudoku::pint;

fn main() {
    let mut s = [[0; 9]; 9];
    basic::sudoku(&mut s);
    pint(&s);
}

#[bench]
fn bench_zeroes_basic(b: &mut Bencher) {
    let s = [[0; 9]; 9];
    use basic::sudoku;
    b.iter(|| {
        sudoku(&mut s.clone())
    });
}

#[bench]
fn bench_zeroes_sorted(b: &mut Bencher) {
    let s = [[0; 9]; 9];
    use sorted::sudoku;
    b.iter(|| {
        sudoku(&mut s.clone())
    });
}

#[bench]
fn bench_zeroes_entries(b: &mut Bencher) {
    let s = [[0; 9]; 9];
    use entries::sudoku;
    b.iter(|| {
        sudoku(&mut s.clone())
    });
}
