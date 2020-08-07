pub mod solvers;

pub type Table = [[usize; 9]; 9];

pub fn pint(table: &Table) {
    for r in table {
        println!("{:?}", r);
    }
}

