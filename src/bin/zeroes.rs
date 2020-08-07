use sudoku::solvers::sorted::sudoku;
use sudoku::pint;

fn main() {
    let mut s = [[0; 9]; 9];
    sudoku(&mut s);
    pint(&s);
}
