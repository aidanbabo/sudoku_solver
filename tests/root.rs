use sudoku::solvers::{basic, entries, sorted};
use sudoku::Table;

const ONLINE: Table = [
    [1, 4, 0, 0, 0, 0, 0, 0, 6],
    [0, 0, 8, 0, 0, 0, 0, 0, 3],
    [7, 0, 0, 4, 0, 0, 0, 0, 5],
    [0, 0, 0, 0, 4, 0, 0, 0, 8],
    [0, 1, 0, 0, 0, 3, 0, 0, 9],
    [4, 0, 9, 0, 6, 5, 0, 0, 2],
    [0, 0, 0, 0, 0, 1, 9, 0, 7],
    [3, 2, 0, 0, 0, 0, 0, 0, 4],
    [9, 8, 7, 6, 5, 4, 3, 2, 1],
];

const JON: Table = [
    [4, 0, 0, 0, 5, 0, 8, 0, 0], 
    [0, 1, 8, 0, 0, 0, 7, 0, 0], 
    [0, 0, 3, 0, 0, 4, 0, 0, 0], 
    [9, 6, 0, 0, 0, 0, 0, 0, 0], 
    [0, 0, 5, 0, 0, 3, 0, 0, 0], 
    [0, 7, 0, 0, 0, 8, 0, 6, 0], 
    [0, 0, 1, 6, 0, 0, 0, 0, 4], 
    [0, 0, 0, 5, 0, 0, 0, 1, 3], 
    [0, 0, 0, 8, 0, 0, 0, 0, 0], 
];

// Tested. Works. Removing test cause it clouds output up
fn is_valid(table: &Table) -> bool {
    const ALL: [usize; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut v: Vec<Vec<usize>> = Vec::with_capacity(27);
    for row in table {
        row.clone().sort();
        let mut inner = Vec::with_capacity(9);
        inner.extend_from_slice(row);
        inner.sort();
        v.push(inner);
    }
    for col in 0..9 {
        let mut inner = Vec::with_capacity(9);
        for row in table {
            inner.push(row[col]);
        }
        inner.sort();
        v.push(inner);
    }
    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            let mut inner = Vec::with_capacity(9);
            for k in 0..3 {
                for l in 0..3 {
                    inner.push(table[i+k][j+l]);
                }
            }
            inner.sort();
            v.push(inner);
        }
    }
    v.iter().all(|l| &ALL == l.as_slice())
}

#[test]
fn online_basic() {
    let mut s = ONLINE;
    basic::sudoku(&mut s);
    assert!(is_valid(&s));
}

#[test]
fn jon_basic() {
    let mut s = JON;
    basic::sudoku(&mut s);
    assert!(is_valid(&s));
}

#[test]
fn online_sorted() {
    let mut s = ONLINE;
    sorted::sudoku(&mut s);
    assert!(is_valid(&s));
}

#[test]
fn jon_sorted() {
    let mut s = JON;
    sorted::sudoku(&mut s);
    assert!(is_valid(&s));
}

#[test]
fn online_entries() {
    let mut s = ONLINE;
    entries::sudoku(&mut s);
    assert!(is_valid(&s));
}

#[test]
fn jon_entries() {
    let mut s = JON;
    entries::sudoku(&mut s);
    assert!(is_valid(&s));
}
