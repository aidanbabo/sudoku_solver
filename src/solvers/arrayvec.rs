use crate::Table;
use arrayvec::ArrayVec;
use std::mem::{self, MaybeUninit};

pub fn solve(table: &mut Table) -> bool {
    let mut entries = Entries::from_table_new(table);
    rec(table, &mut entries)
}

fn rec(table: &mut Table, entries: &mut Entries) -> bool {
    if let Some((y, x, v)) = entries.min() {
        let taken = entries.take_entry_possibles(y, x);
        for p in v {
            table[y][x] = p;
            let replacements = entries.remove(y, x, p);
            if rec(table, entries) {
                return true;
            }
            table[y][x] = 0;
            entries.add_back(replacements, p);
        }
        entries.replace_possibles(y, x, taken);
        false
    } else {
        true
    }
}

type ArrayVec9<T> = ArrayVec<[T; 9]>;
type Entry = Option<ArrayVec9<usize>>;

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
struct Entries {
    entries: [[Entry; 9]; 9],
}

impl Entries {
    pub fn from_table_new(table: &Table) -> Self {
        let mut entries: [[MaybeUninit<Entry>; 9]; 9] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..entries.len() {
            for j in 0..entries[i].len() {
                entries[i][j] = MaybeUninit::new(Some(ArrayVec::from([1, 2, 3, 4, 5, 6, 7, 8, 9])));
            }
        }

        let mut entries: [[Entry; 9]; 9] = unsafe { mem::transmute(entries) };

        #[inline]
        fn remove(entries: &mut [[Entry; 9]; 9], row: usize, col: usize, possible: usize) {
            if let Some(ref mut v) = entries[row][col] {
                if let Some(index) = v.iter().position(|&x| x == possible) {
                    v.remove(index);
                }
            }
        }

        for row in 0..entries.len() {
            for col in 0..entries[row].len() {
                let value = table[row][col];
                if value != 0 {
                    entries[row][col] = None;
                    for i in 0..9 {
                        remove(&mut entries, row, i, value);
                        remove(&mut entries, i, col, value);
                    }
                    let y = row / 3 * 3;
                    let x = col / 3 * 3;
                    for i in 0..3 {
                        for j in 0..3 {
                            remove(&mut entries, y + i, x + j, value);
                        }
                    }
                }
            }
        }

        Entries { entries }
    }

    pub fn take_entry_possibles(&mut self, row: usize, col: usize) -> Option<ArrayVec9<usize>> {
        self.entries[row][col].take()
    }

    pub fn replace_possibles(
        &mut self,
        row: usize,
        col: usize,
        possibles: Option<ArrayVec9<usize>>,
    ) {
        self.entries[row][col] = possibles;
    }

    pub fn add_back(&mut self, changed: ArrayVec<[(usize, usize); 22]>, n: usize) {
        for (row, col) in changed {
            if let Some(ref mut v) = self.entries[row][col] {
                v.push(n);
            }
        }
    }

    pub fn remove(&mut self, row: usize, col: usize, n: usize) -> ArrayVec<[(usize, usize); 22]> {
        let mut changed = ArrayVec::new();
        for i in 0..9 {
            if self.remove_possible_at(row, i, n) {
                changed.push((row, i));
            }
            if self.remove_possible_at(i, col, n) {
                changed.push((i, col));
            }
        }
        let y = row / 3 * 3;
        let x = col / 3 * 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.remove_possible_at(y + i, x + j, n) {
                    changed.push((y + i, x + j));
                }
            }
        }
        changed
    }

    fn remove_possible_at(&mut self, row: usize, col: usize, possible: usize) -> bool {
        if let Some(ref mut v) = self.entries[row][col] {
            if let Some(index) = v.iter().position(|&x| x == possible) {
                v.remove(index);
                return true;
            }
        }
        false
    }

    pub fn min(&self) -> Option<(usize, usize, ArrayVec9<usize>)> {
        let mut min = 10; // You can only have 9 possible
        let mut entry = None;
        for i in 0..self.entries.len() {
            for j in 0..self.entries[i].len() {
                if let Some(ref e) = self.entries[i][j] {
                    if e.len() < min {
                        min = e.len();
                        entry = Some((i, j, e));
                    }
                }
            }
        }

        entry.map(|(row, col, entry)| (row, col, entry.clone()))
    }
}

#[cfg(test)]
mod test {

    /*
    use super::*;
    use crate::ONLINE;

    #[test]
    fn from_tables() {
        let entries_old = Entries::from_table_old(&ONLINE);
        let entries_new = Entries::from_table_new(&ONLINE);
        assert_eq!(entries_old, entries_new);
    }
    */
}
