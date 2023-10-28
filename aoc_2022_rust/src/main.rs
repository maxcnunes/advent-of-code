use clap::Parser;
use error::SolutionError;
use std::fs::File;
use std::io::{self};

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

// project mods
mod error;
mod icon;
mod input;

// days mods
mod day01;
mod day02;
mod day03;

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

        let lines = input::load(self.day, demo)?;

        (self.internal_process)(lines)
    }
}

fn main() {
    let args = Args::parse();

    let solutions = vec![
        // Day 1
        Solution::new(1, 1, 1, day01::part01::process),
        // Day 2
        Solution::new(2, 1, 1, day02::part01::process),
        Solution::new(2, 1, 2, day02::part01_v2::process),
        Solution::new(2, 2, 1, day02::part02::process),
        // Day 3
        Solution::new(3, 1, 1, day03::part01::process),
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
