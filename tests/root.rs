use sudoku::solvers::{arrayvec, basic, entries, sorted};
use sudoku::{Table, JON, ONLINE};

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
                    inner.push(table[i + k][j + l]);
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

#[test]
fn online_arrayvec() {
    let mut s = ONLINE;
    arrayvec::solve(&mut s);
    assert!(is_valid(&s));
}

#[test]
fn jon_arrayvec() {
    let mut s = JON;
    arrayvec::solve(&mut s);
    assert!(is_valid(&s));
}

// They aren't all necessarily equal
/*
#[test]
fn online_all() {
    let mut basic = ONLINE;
    let mut sorted = ONLINE;
    let mut entries = ONLINE;
    let mut arrayvec = ONLINE;
    basic::solve(&mut basic);
    sorted::solve(&mut sorted);
    entries::solve(&mut entries);
    arrayvec::solve(&mut arrayvec);
    assert_eq!(basic, sorted, "basic and sorted where not equal");
    assert_eq!(sorted, entries, "sorted and entries where not equal");
    assert_eq!(entries, arrayvec, "entries and arrayvec where not equal");
    assert_eq!(arrayvec, basic, "arrayvec and basic where not equal");
}

#[test]
fn jon_all() {
    let mut basic = JON;
    let mut sorted = JON;
    let mut entries = JON;
    let mut arrayvec = JON;
    basic::solve(&mut basic);
    sorted::solve(&mut sorted);
    entries::solve(&mut entries);
    arrayvec::solve(&mut arrayvec);
    assert_eq!(basic, sorted, "basic and sorted where not equal");
    assert_eq!(sorted, entries, "sorted and entries where not equal");
    assert_eq!(entries, arrayvec, "entries and arrayvec where not equal");
    assert_eq!(arrayvec, basic, "arrayvec and basic where not equal");
}
*/
