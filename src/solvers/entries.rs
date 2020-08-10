use std::mem::{self, MaybeUninit};
use crate::{Location, Table};
use crate::solvers::basic::Possibles;

pub fn solve(table: &mut Table) -> bool {
    let mut entries = Entries::from_table(table);
    rec(table, &mut entries)
}

fn rec(table: &mut Table, entries: &mut Entries) -> bool {
    if let Some((y, x, v)) = entries.min() {
        let taken = entries.take_entry_possibles((y, x));
        for p in v {
            table[y][x] = p; 
            let replacements = entries.remove((y, x), p);
            if rec(table, entries) {
                return true;
            }
            table[y][x] = 0;
            entries.add_back(replacements, p);
        }
        entries.replace_possibles((y, x), taken);
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
                            MaybeUninit::new(Entry { possibles: Some(Possibles::iter(table.clone(), (i, j)).collect()) })
                        } else {
                            MaybeUninit::new(Entry { possibles: None })
                        }
                    }
                }
                unsafe { mem::transmute::<_, [[Entry; 9]; 9]>(entries) }
            },
        }
    }

    pub fn take_entry_possibles(&mut self, (row, col): Location) -> Option<Vec<usize>> {
        self.entries[row][col].possibles.take()
    }

    pub fn replace_possibles(&mut self, (row, col): Location, possibles: Option<Vec<usize>>) {
        self.entries[row][col].possibles = possibles;
    }

    pub fn add_back(&mut self, changed: Vec<Location>, n: usize) {
        for (row, col) in changed {
            let mut replacement = self.entries[row][col].possibles.take().unwrap();
            replacement.push(n);
            self.entries[row][col].possibles = Some(replacement);
        }
    }

    pub fn remove(&mut self, (row, col): Location, n: usize) -> Vec<Location> {
        let mut changed = Vec::new();
        for i in 0..9 {
            if let Some(ref mut v) = self.entries[row][i].possibles {
                if let Some(index) = v.iter().position(|&x| x == n) {
                    changed.push((row, i));
                    v.remove(index);
                }
            };
            if let Some(ref mut v) = self.entries[i][col].possibles {
                if let Some(index) = v.iter().position(|&x| x == n) {
                    changed.push((i, col));
                    v.remove(index);
                }
            };
        }
        let y = row / 3 * 3;
        let x = col / 3 * 3;
        for i in 0..3 {
            for j in 0..3 {
                if let Some(ref mut v) = self.entries[y+i][x+j].possibles {
                    if let Some(index) = v.iter().position(|&x| x == n) {
                        changed.push((y+i, x+j));
                        v.remove(index);
                    }
                };
            }
        }
        changed
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
