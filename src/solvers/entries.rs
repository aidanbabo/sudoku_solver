use std::mem::{self, MaybeUninit};
use crate::Table;

pub fn sudoku(table: &mut Table) -> bool {
    let mut entries = Entries::from_table(table);
    solve(table, &mut entries)
}

fn solve(table: &mut Table, entries: &mut Entries) -> bool {
    if let Some((y, x, v)) = entries.min() {
        for p in v {
            table[y][x] = p; 
            // `entries` here
            entries.update(table, (y, x));
            if solve(table, entries) {
                return true;
            }
            table[y][x] = 0;
            // is equal to `entries` here
            entries.update(table, (y, x)); 
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
        // Some unsafe code to incrementally initialize the array
        Entries {
            entries: {
                let mut entries: [[MaybeUninit<Entry>; 9]; 9] = unsafe { MaybeUninit::uninit().assume_init() };
                for i in 0..entries.len() {
                    for j in 0..entries[i].len() {
                        entries[i][j] = if table[i][j] == 0 {
                            MaybeUninit::new(Entry { possibles: Some(possibles(table, i, j)) })
                        } else {
                            MaybeUninit::new(Entry { possibles: None })
                        }
                    }
                }
                unsafe { mem::transmute::<_, [[Entry; 9]; 9]>(entries) }
            },
        }
    }

    // Lot of duplicate calls to update possibles
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
            Some(possibles(table, row, col))
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

pub fn possibles(table: &Table, y: usize, x: usize) -> Vec<usize> {
    let mut v = Vec::new();
    // [y][x] is 0
    for i in 0..9 {
        if table[y][i] != 0 {
            v.push(table[y][i]);
        }
        if table[i][x] != 0 {
            v.push(table[i][x]);
        }
    }
    let sy = y / 3;
    let sx = x / 3;
    // let py = y % 3;
    // let py = y % 3;
    for i in 0..3 {
        for j in 0..3 {
            if table[sy+i][sx+j] != 0 {
                v.push(table[sy+i][sx+j])
            }
        }
    }
    v.dedup();
    v
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
