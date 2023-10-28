use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::error::SolutionError;

pub fn load(day: u8, demo: bool) -> Result<io::Lines<io::BufReader<File>>, SolutionError> {
    let day_prefix = match day < 10 {
        true => "0",
        false => "",
    };

    let day_label = format!("{}{}", day_prefix, day);

    let file_name = match demo {
        true => format!("src/day{}/demo-input.txt", day_label),
        false => format!("src/day{}/input.txt", day_label),
    };

    let lines = read_lines(file_name).map_err(SolutionError::ReadFileErr)?;

    Ok(lines)
}

// Uses a buffer and iterator to read a file instead of loading the whole
// file in memory at once.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // println!("Reading file {:?}", filename);
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
