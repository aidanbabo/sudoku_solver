use crate::Table;

pub fn sudoku(table: &mut Table) -> bool {
    for y in 0..9 {
        for x in 0..9 {
            if table[y][x] == 0 {
                for p in Possibles::iter(table.clone(), y, x) {
                // for p in possibles(table, y, x) {
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

pub(crate) fn possibles(table: &Table, y: usize, x: usize) -> Vec<usize> {
    let mut seen = Vec::with_capacity(9);
    // [y][x] is 0
    for i in 0..9 {
        if table[y][i] != 0 {
            seen.push(table[y][i]);
        }
        if table[i][x] != 0 {
            seen.push(table[i][x]);
        }
    }
    let sy = y / 3;
    let sx = x / 3;
    // let py = y % 3;
    // let py = y % 3;
    // Until we only look at each square once, we need another if
    for i in 0..3 {
        for j in 0..3 {
            if table[sy+i][sx+j] != 0 {
                if !seen.contains(&table[sy+i][sx+j]) {
                    seen.push(table[sy+i][sx+j])
                }
            }
        }
    }
    (1..9).filter(|x| !seen.contains(x)).collect()
}

#[test]
fn possibles_works() {
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
    let ps = possibles(&ONLINE, 0, 2);
    assert_eq!([2, 3, 5], *ps);
}

#[test]
#[allow(non_snake_case)]
fn possible_is_Possibles() {
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
    let ps = possibles(&ONLINE, 0, 2);
    let pps: Vec<_> = Possibles::iter(ONLINE, 0, 2).collect();
    assert_eq!(ps, pps);
}

pub(crate) struct Possibles {
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
