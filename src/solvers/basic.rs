use super::{possibles, Table};

pub fn solve(table: &mut Table) -> bool {
    for y in 0..9 {
        for x in 0..9 {
            if table[y][x] == 0 {
                for p in possibles(table, (y, x)) {
                    table[y][x] = p;
                    if solve(table) {
                        return true;
                    }
                    table[y][x] = 0;
                }
                return false;
            }
        }
    }
    true
}
