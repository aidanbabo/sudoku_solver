use sudoku::solvers::{basic, entries, sorted};
use sudoku::{ONLINE, JON, Table};

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
    basic::solve(&mut s);
    assert!(is_valid(&s));
}

#[test]
fn jon_basic() {
    let mut s = JON;
    basic::solve(&mut s);
    assert!(is_valid(&s));
}

#[test]
fn online_sorted() {
    let mut s = ONLINE;
    sorted::solve(&mut s);
    assert!(is_valid(&s));
}

#[test]
fn jon_sorted() {
    let mut s = JON;
    sorted::solve(&mut s);
    assert!(is_valid(&s));
}

#[test]
fn online_entries() {
    let mut s = ONLINE;
    entries::solve(&mut s);
    assert!(is_valid(&s));
}

#[test]
fn jon_entries() {
    let mut s = JON;
    entries::solve(&mut s);
    assert!(is_valid(&s));
}
