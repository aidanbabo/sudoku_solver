use crate::Table;
use crate::solvers::basic::Possibles;

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
                buf.extend(Possibles::iter(table.clone(), y, x));
                if buf.len() < length {
                    length = buf.len();
                    ret = Some((y, x, buf.drain(..).collect()));
                }
            }
        }
    }
    ret
}
