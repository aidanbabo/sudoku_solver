use std::mem::{self, MaybeUninit};
use crate::{Location, Table};
use crate::solvers::basic::Possibles;

pub fn solve(table: &mut Table) -> bool {
    let mut entries = Entries::from_table(table);
    rec(table, &mut entries)
}

fn rec(table: &mut Table, entries: &mut Entries) -> bool {
    if let Some((y, x, v)) = entries.min() {
        for p in v {
            table[y][x] = p; 
            let replacements = entries.remove((y, x), p);
            if rec(table, entries) {
                return true;
            }
            table[y][x] = 0;
            entries.add_back(replacements, p);
        }
        false
    } else {
        true
    }
}

struct Replacements {
    focus_location: Location,
    focus_inner: Vec<usize>,
    changed: Vec<Location>,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
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

    pub fn add_back(&mut self, replacements: Replacements, n: usize) {
        let Replacements { focus_location: (row, col), focus_inner, changed } = replacements;
        self.entries[row][col].possibles = Some(focus_inner);

        for (row, col) in changed {
            let mut replacement = self.entries[row][col].possibles.take().unwrap();
            replacement.push(n);
            self.entries[row][col].possibles = Some(replacement);
        }
    }

    pub fn remove(&mut self, (row, col): Location, n: usize) -> Replacements {
        let focus_inner = self.entries[row][col].possibles.take().unwrap();
        
        let mut changed = Vec::new();
        for i in 0..9 {
            self.entries[row][i].possibles = match self.entries[row][i].possibles.take() {
                Some(mut v) => {
                    if let Some(index) = v.iter().position(|&x| x == n) {
                        changed.push((row, i));
                        v.remove(index);
                        Some(v)
                    } else {
                        Some(v)
                    }
                },
                None => None,
            };
            self.entries[i][col].possibles = match self.entries[i][col].possibles.take() {
                Some(mut v) => {
                    if let Some(index) = v.iter().position(|&x| x == n) {
                        changed.push((i, col));
                        v.remove(index);
                        Some(v)
                    } else {
                        Some(v)
                    }
                },
                None => None,
            };
        }
        let y = row / 3 * 3;
        let x = col / 3 * 3;
        for i in 0..3 {
            for j in 0..3 {
                self.entries[y+i][x+j].possibles = match self.entries[y+i][x+j].possibles.take() {
                    Some(mut v) => {
                        if let Some(index) = v.iter().position(|&x| x == n) {
                            changed.push((y+i, x+j));
                            v.remove(index);
                            Some(v)
                        } else {
                            Some(v)
                        }
                    },
                    None => None,
                };
            }
        }
        Replacements {
            focus_location: (row, col),
            focus_inner,
            changed,
        }
    }

    #[cfg(test)]
    pub fn update(&mut self, table: &Table, (row, col): Location) {
        for i in 0..9 {
            self.update_possibles(table, (row, i));
            self.update_possibles(table, (i, col));
        }
        let y = row / 3 * 3;
        let x = col / 3 * 3;
        for i in 0..3 {
            for j in 0..3 {
                self.update_possibles(table, (y+i, x+j));
            }
        }
    }

    #[cfg(test)]
    fn update_possibles(&mut self, table: &Table, (row, col): Location) {
        self.entries[row][col].possibles = if table[row][col] == 0 {
            Some(Possibles::iter(table.clone(), (row, col)).collect())
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

#[cfg_attr(test, derive(Debug))]
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
impl PartialEq<Entry> for Entry {
    fn eq(&self, other: &Entry) -> bool {
        let s = self.possibles.clone().map(|mut ps| ps.sort());
        let o = other.possibles.clone().map(|mut ps| ps.sort());
        if s == o {
            true
        } else {
            println!("Not equal!");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ONLINE;

    #[test]
    fn remove_and_update() {
        let mut s = Entries::from_table(&ONLINE);
        let mut t = Entries::from_table(&ONLINE);
        const ROW: usize = 0;
        const COL: usize = 2;
        let mut ps = Possibles::iter(ONLINE, (ROW, COL));
        let n = ps.next().unwrap();
        let mut online = ONLINE;
        online[ROW][COL] = n;
        s.update(&online, (ROW, COL));
        t.remove((ROW, COL), n);
        assert_eq!(s, t);
    }

    #[test]
    fn add_and_update() {
        let mut s = Entries::from_table(&ONLINE);
        let mut t = Entries::from_table(&ONLINE);
        const ROW: usize = 0;
        const COL: usize = 2;
        let mut ps = Possibles::iter(ONLINE, (ROW, COL));
        let n = ps.next().unwrap();
        let mut online = ONLINE;
        online[ROW][COL] = n;
        s.update(&online, (ROW, COL));
        let replacements = t.remove((ROW, COL), n);
        // Add back
        online[ROW][COL] = 0;
        s.update(&online, (ROW, COL));
        t.add_back(replacements, n);
        assert_eq!(s, t);
    }
}
