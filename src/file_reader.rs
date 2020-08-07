use std::io;
use std::io::prelude::*;
use std::fs::File;

use crate::Table;

pub struct Sudokus {
    reader: io::BufReader<File>,
    remaining: usize,
}

impl Sudokus {
    pub fn from_file(file_name: &'static str) -> io::Result<Self> {
        let f = File::open(file_name)?;
        let mut reader = io::BufReader::new(f);
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;
        let remaining = buffer.trim().parse::<usize>().unwrap();
        Ok(Sudokus {
            reader,
            remaining,
        })
    }
}

impl Iterator for Sudokus {
    type Item = Table;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            self.remaining -= 1;
            let mut table = [[0; 9]; 9];
            for i in 0..9 {
                let mut buffer = String::new();
                match self.reader.read_line(&mut buffer) {
                    Ok(0) if i < 8 => {
                        println!("Reached end of file too quickly."); 
                        return None;
                    },
                    Ok(0) => println!("Reached end of file."),
                    _ => {},
                }
                let v: Vec<usize> = buffer.trim().split(" ").map(|s| s.parse().unwrap()).collect();
                let mut a = [0; 9];
                a.copy_from_slice(&v[..9]);
                table[i] = a;
            }
            Some(table)
        } else {
            None
        }
    }
}
