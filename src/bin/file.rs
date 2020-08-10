use sudoku::solvers::entries::solve;
use sudoku::pint;
use sudoku::file_reader::Sudokus;

use std::io;

fn main() -> io::Result<()> {
    let sudokus = Sudokus::from_file("src/sudokus.txt")?;
    for mut s in sudokus {
        solve(&mut s);
        pint(&s);
        println!();
    }
    Ok(())
}
