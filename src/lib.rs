pub mod solvers;
pub mod file_reader;

pub type Table = [[usize; 9]; 9];
type Location = (usize, usize);

pub fn pint(table: &Table) {
    for r in table {
        println!("{:?}", r);
    }
}

pub const ONLINE: Table = [
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

pub const JON: Table = [
    [4, 0, 0, 0, 5, 0, 8, 0, 0], 
    [0, 1, 8, 0, 0, 0, 7, 0, 0], 
    [0, 0, 3, 0, 0, 4, 0, 0, 0], 
    [9, 6, 0, 0, 0, 0, 0, 0, 0], 
    [0, 0, 5, 0, 0, 3, 0, 0, 0], 
    [0, 7, 0, 0, 0, 8, 0, 6, 0], 
    [0, 0, 1, 6, 0, 0, 0, 0, 4], 
    [0, 0, 0, 5, 0, 0, 0, 1, 3], 
    [0, 0, 0, 8, 0, 0, 0, 0, 0], 
];

