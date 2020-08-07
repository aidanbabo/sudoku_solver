type Table = [[usize; 9]; 9];

pub fn sudoku(table: &mut Table) -> bool {
    for y in 0..9 {
        for x in 0..9 {
            if table[y][x] == 0 {
                for p in 1..=9 {
                    if possible(table, y, x, p) {
                        table[y][x] = p;
                        if sudoku(table) {
                            return true;
                        }
                        table[y][x] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
}

pub fn pint(table: &Table) {
    for r in table {
        println!("{:?}", r);
    }
}

fn possible(table: &Table, y: usize, x: usize, p: usize) -> bool {
    for i in 0..9 {
        if table[y][i] == p {
            return false;
        }
        if table[i][x] == p {
            return false;
        }
    }
    let x = x / 3 * 3;
    let y = y / 3 * 3;
    for i in 0..3 {
        for j in 0..3 {
            if table[y+i][x+j] == p {
                return false;
            }
        }
    }
    true
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

    #[test]
    fn check_row() {
        assert!(!possible(&TABLE, 1, 2, 4));
    }

    #[test]
    fn check_col() {
        assert!(!possible(&TABLE, 0, 1, 5));
    }

    #[test]
    fn check_square() {
        assert!(!possible(&TABLE, 0, 1, 2));
    }

    #[test]
    fn jon() {
        assert!(sudoku(&mut TABLE));
    }
}
