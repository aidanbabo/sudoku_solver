use crate::Table;

pub fn sudoku(table: &mut Table) -> bool {
    if let Some((y, x, v)) = next_best(table) {
        for p in v {
            table[y][x] = p;
            if sudoku(table) {
                return true;
            }
            table[y][x] = 0;
        }
        false
    } else {
        true
    }
}

// TODO it would be cool to return a `Possibles` struct, to save space
// This would probably result in cloning one, instead of allocating it's
// contents into a Vec<>, which is better?
fn next_best(table: &Table) -> Option<(usize, usize, Vec<usize>)> {
    let mut length = usize::MAX;
    let mut ret = None;
    let mut buf = Vec::with_capacity(9);
    for y in 0..9 {
        for x in 0..9 {
            if table[y][x] == 0 {
                buf.clear();
                buf.extend(Possibles::iter(table, y, x));
                if buf.len() < length {
                    length = buf.len();
                    ret = Some((y, x, buf.drain(..).collect()));
                }
            }
        }
    }
    ret
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
