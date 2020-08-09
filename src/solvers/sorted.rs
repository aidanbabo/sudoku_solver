use crate::Table;
use crate::solvers::entries::possibles;

pub fn sudoku(table: &mut Table) -> bool {
    if let Some((y, x, v)) = next_best(table) {
        for p in v {
            table[y][x] = p;
            if sudoku(table) {
                return true;
            }
            table[y][x] = 0;
        }
        false
    } else {
        true
    }
}

fn next_best(table: &Table) -> Option<(usize, usize, Vec<usize>)> {
    let mut length = usize::MAX;
    let mut ret = None;
    let mut buf = Vec::with_capacity(9);
    for y in 0..9 {
        for x in 0..9 {
            if table[y][x] == 0 {
                buf.clear();
                buf.extend(possibles(table, y, x));
                if buf.len() < length {
                    length = buf.len();
                    ret = Some((y, x, buf.drain(..).collect()));
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE: Table = [
        [4, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 8, 0, 9, 0, 0, 0, 4, 0],
        [0, 1, 0, 0, 0, 3, 5, 0, 0],
        [0, 0, 3, 2, 1, 0, 0, 0, 0],
        [0, 0, 4, 7, 0, 5, 2, 0, 0],
        [0, 0, 0, 0, 9, 8, 3, 0, 0],
        [0, 0, 8, 1, 0, 0, 0, 3, 0],
        [0, 5, 0, 0, 0, 4, 0, 8, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 9],
    ];

    const JON_HARD: Table = [
        [4,0,0,0,5,0,8,0,0],
        [0,1,8,0,0,0,7,0,0],
        [0,0,3,0,0,4,0,0,0],
        [9,6,0,0,0,0,0,0,0],
        [0,0,5,0,0,3,0,0,0],
        [0,7,0,0,0,8,0,6,0],
        [0,0,1,6,0,0,0,0,4],
        [0,0,0,5,0,0,0,1,3],
        [0,0,0,8,0,0,0,0,0],
    ];

    #[test]
    fn jon() {
        assert!(sudoku(&mut TABLE));
    }

    #[test]
    fn jon_hard() {
        assert!(sudoku(&mut JON_HARD));
    }
}
