pub mod arrayvec;
pub mod basic;
pub mod entries;
pub mod sorted;

use crate::{Location, Table};
use std::iter;

#[cfg(test)]
pub(crate) fn possibles_vec(table: Table, (y, x): Location) -> Vec<usize> {
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
    let y = y / 3 * 3;
    let x = x / 3 * 3;
    for i in 0..3 {
        for j in 0..3 {
            if table[y + i][x + j] != 0 {
                if !seen.contains(&table[y + i][x + j]) {
                    seen.push(table[y + i][x + j])
                }
            }
        }
    }
    (1..10).filter(|x| !seen.contains(x)).collect()
}

pub(crate) fn possibles(table: &Table, (y, x): Location) -> impl Iterator<Item = usize> {
    let mut seen = [0; 9];
    for i in 0..9 {
        if table[y][i] != 0 {
            seen[table[y][i] - 1] += 1;
        }
        if table[i][x] != 0 {
            seen[table[i][x] - 1] += 1;
        }
    }
    let y = y / 3 * 3;
    let x = x / 3 * 3;
    for i in 0..3 {
        for j in 0..3 {
            let entry = table[y + i][x + j];
            if entry != 0 {
                seen[entry - 1] += 1;
            }
        }
    }

    let mut index = 0;
    iter::from_fn(move || {
        while index < seen.len() {
            if seen[index] == 0 {
                index += 1;
                return Some(index);
            }
            index += 1;
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possibles_vec_works() {
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
        let ps = possibles_vec(ONLINE, (0, 2));
        assert_eq!([2, 3, 5], *ps);
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
        let ps: Vec<_> = possibles(&ONLINE, (0, 2)).collect();
        assert_eq!([2, 3, 5], *ps);
    }

    #[test]
    fn possibles_and_possibles_vec_equivalent() {
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
        for i in 0..9 {
            for j in 0..9 {
                let ps: Vec<_> = possibles_vec(ONLINE, (i, j));
                let psi: Vec<_> = possibles(&ONLINE, (i, j)).collect();
                assert_eq!(ps, psi);
            }
        }
    }
}
