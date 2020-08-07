type Table = [[usize; 9]; 9];

pub fn sudoku(table: &mut Table) -> bool {
    for y in 0..9 {
        for x in 0..9 {
            if table[y][x] == 0 {
                for p in Possibles::iter(table.clone(), y, x) {
                    table[y][x] = p;
                    if sudoku(table) {
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

pub fn pint(table: &Table) {
    for r in table {
        println!("{:?}", r);
    }
}

struct Possibles {
    current: usize,
    table: Table,
    y: usize,
    x: usize,
}

impl Possibles {
    pub fn iter(table: Table, y: usize, x: usize) -> Self {
        Possibles {
            current: 1,
            table,
            y,
            x,
        }
    }

    fn possible(&self, p: usize) -> bool {
        for i in 0..9 {
            if self.table[self.y][i] == p {
                return false;
            }
            if self.table[i][self.x] == p {
                return false;
            }
        }
        let x = self.x / 3 * 3;
        let y = self.y / 3 * 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.table[y+i][x+j] == p {
                    return false;
                }
            }
        }
        true
    }
}

impl Iterator for Possibles {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while self.current <= 9 {
            if self.possible(self.current) {
                let next = self.current;
                self.current += 1;
                return Some(next);
            } else {
                self.current += 1;
            }
        }
        None
    }
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

    /*
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
    */

    #[test]
    fn jon() {
        assert!(sudoku(&mut TABLE));
    }
}
