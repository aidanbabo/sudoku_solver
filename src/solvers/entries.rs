use std::mem::{self, MaybeUninit};
use crate::Table;
use crate::solvers::basic::Possibles;

pub fn sudoku(table: &mut Table) -> bool {
    let mut entries = Entries::from_table(table);
    solve(table, &mut entries)
}

fn solve(table: &mut Table, entries: &mut Entries) -> bool {
    if let Some((y, x, v)) = entries.min() {
        for p in v {
            table[y][x] = p; 
            entries.update(table, (y, x));
            if solve(table, entries) {
                return true;
            }
            table[y][x] = 0;
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
                            MaybeUninit::new(Entry { possibles: Some(Possibles::iter(table.clone(), i, j).collect()) })
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
            Some(Possibles::iter(table.clone(), row, col).collect())
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
