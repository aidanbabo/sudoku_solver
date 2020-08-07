use crate::Table;

pub fn sudoku(table: &mut Table) -> bool {
    let mut entries = Entries::from_table(table);
    solve(table, &mut entries)
}

fn solve(table: &mut Table, entries: &mut Entries) -> bool {
    if let Some((y, x, v)) = entries.min() {
        for p in v {
            table[y][x] = p; // `entries` here
            entries.update(table, (y, x));
            if solve(table, entries) {
                return true;
            }
            table[y][x] = 0;
            entries.update(table, (y, x)); // is equal to `entries` here
        }
        false
    } else {
        true
    }
}

struct Entries {
    entries: [[Entry; 9]; 9],
}

impl Entries {
    pub fn from_table(table: &Table) -> Self {
        use std::mem::{self, MaybeUninit};
        // Some unsafe code to incrementally initialize the array
        Entries {
            entries: {
                let mut entries: [[MaybeUninit<Entry>; 9]; 9] = unsafe {
                    MaybeUninit::uninit().assume_init()
                };
                for i in 0..entries.len() {
                    for j in 0..entries[i].len() {
                        entries[i][j] = if table[i][j] == 0 {
                            MaybeUninit::new(Entry { possibles: Some(Possibles::iter(table, i, j).collect()) })
                        } else {
                            MaybeUninit::new(Entry { possibles: None })
                        }
                    }
                }
                unsafe { mem::transmute::<_, [[Entry; 9]; 9]>(entries) }
            },
        }
    }

    pub fn update(&mut self, table: &Table, (row, col): (usize, usize)) {
        for i in 0..9 {
            self.update_possibles(table, row, i);
            self.update_possibles(table, i, col);
        }
        let y = row / 3 * 3;
        let x = col / 3 * 3;
        for i in 0..3 {
            for j in 0..3 {
                self.update_possibles(table, y+i, x+j);
            }
        }
    }

    fn update_possibles(&mut self, table: &Table, row: usize, col: usize) {
        self.entries[row][col].possibles = if table[row][col] == 0 {
            Some(Possibles::iter(table, row, col).collect())
        } else {
            None
        }
    }

    pub fn min(&self) -> Option<(usize, usize, Vec<usize>)> {
        let mut min = usize::MAX;
        let mut inds = None;
        for i in 0..self.entries.len() {
            for j in 0..self.entries[i].len() {
                let ref e = self.entries[i][j];
                let len = e.len();
                if len < min {
                    min = len;
                    inds = Some((i, j));
                }
            }
        }
        // avoid cloning?
        inds.map(|(i, j)| (i, j, self.entries[i][j].possibles.clone().unwrap()))
    }
}

struct Entry {
    possibles: Option<Vec<usize>>,
}

impl Entry {
    pub fn len(&self) -> usize {
        match self.possibles {
            Some(ref v) => v.len(),
            None => usize::MAX,
        }
    }
}

struct Possibles<'a> {
    current: usize,
    table: &'a Table,
    y: usize,
    x: usize,
}

impl<'a> Possibles<'a> {
    pub fn iter(table: &'a Table, y: usize, x: usize) -> Self {
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

impl<'a> Iterator for Possibles<'a> {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(9))
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
