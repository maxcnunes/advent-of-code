use clap::Parser;

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

struct Solution {
    day: u8,
    part: u8,
    version: u8,

    internal_process: fn(demo: bool),
}

impl Solution {
    fn new(day: u8, part: u8, version: u8, internal_process: fn(demo: bool)) -> Solution {
        Solution {
            day,
            part,
            version,
            internal_process,
        }
    }

    // TODO: Use Result instead of panic
    fn run(&self, demo: bool) {
        println!(
            "\nRunning day={} part={} version={} demo={}",
            self.day, self.part, self.version, demo
        );

        (self.internal_process)(demo);
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

        s.run(args.demo);
        return;
    }

    println!("Running all {} solutions", solutions.len());

    for s in solutions {
        s.run(args.demo);
    }
}
