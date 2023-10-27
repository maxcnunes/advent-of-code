use clap::Parser;
use error::SolutionError;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: u8,

    #[arg(short, long, default_value_t = 1)]
    part: u8,

    #[arg(long, default_value_t = 1)]
    part_version: u8,

    #[arg(long)]
    demo: bool,
}

mod day01;
mod day02;
mod error;
mod icon;

struct Solution {
    day: u8,
    part: u8,
    version: u8,
    internal_process: fn(lines: io::Lines<io::BufReader<File>>) -> Result<(), SolutionError>,
}

impl Solution {
    fn new(
        day: u8,
        part: u8,
        version: u8,
        internal_process: fn(lines: io::Lines<io::BufReader<File>>) -> Result<(), SolutionError>,
    ) -> Solution {
        Solution {
            day,
            part,
            version,
            internal_process,
        }
    }

    fn run(&self, demo: bool) -> Result<(), SolutionError> {
        println!(
            "\n{} Running day={} part={} version={} demo={}",
            icon::BULLET,
            self.day,
            self.part,
            self.version,
            demo
        );

        let day_prefix = match self.day < 10 {
            true => "0",
            false => "",
        };

        let day_label = format!("{}{}", day_prefix, self.day);

        let file_name = match demo {
            true => format!("src/day{}/demo-input.txt", day_label),
            false => format!("src/day{}/demo-input.txt", day_label),
        };

        let lines = read_lines(file_name).map_err(SolutionError::ReadFileErr)?;

        (self.internal_process)(lines)
    }
}

fn main() {
    let args = Args::parse();

    let solutions = vec![
        Solution::new(1, 1, 1, day01::part01::process),
        Solution::new(2, 1, 1, day02::part01::process),
        Solution::new(2, 1, 2, day02::part01_v2::process),
        Solution::new(2, 2, 1, day02::part02::process),
    ];

    if args.day != 0 {
        let s = solutions
            .iter()
            .find(|&s| s.day == args.day && s.part == args.part && s.version == args.part_version)
            .expect("Could not run solution, wrong arguments");

        s.run(args.demo).unwrap();
        return;
    }

    println!("Running all {} solutions", solutions.len());

    for s in solutions {
        s.run(args.demo).unwrap();
    }
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
