use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

use grid::Grid;

mod error;
mod grid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: refactor this code
    let args = env::args().collect::<Vec<String>>();
    let file = fs::File::open(&args[1])?;
    let reader = io::BufReader::new(file);

    let mut grid = Grid::parse(reader.lines())?;
    println!("{}", grid);
    grid.solve()?;

    Ok(())
}