use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = load_file(false);

    let total = resolve_total_calibrations(lines);

    println!(
        "Day 01, Part 01: The sum of all of the calibration values: {}",
        total
    );
}

fn resolve_total_calibrations(lines: io::Lines<io::BufReader<File>>) -> usize {
    let calibrations = resolve_calibrations(lines);

    calibrations.iter().fold(0, |acc, n| acc + n)
}

fn resolve_calibrations(lines: io::Lines<io::BufReader<File>>) -> Vec<usize> {
    let mut calibrations: Vec<usize> = vec![];

    for line_result in lines {
        let line = line_result.unwrap();

        // Get first num
        let mut first_num: Option<usize> = None;
        for c in line.chars() {
            if let Ok(num) = c.to_string().parse::<usize>() {
                first_num = Some(num);
                break;
            }
        }

        if let None = first_num {
            panic!("could not find first calibration value");
        }

        // Get last num
        let mut last_num: Option<usize> = None;
        for c in line.chars().rev() {
            if let Ok(num) = c.to_string().parse::<usize>() {
                last_num = Some(num);
                break;
            }
        }

        if let None = last_num {
            panic!("could not find last calibration value");
        }

        let calibration_str = first_num.unwrap().to_string() + &last_num.unwrap().to_string();

        let calibration = calibration_str.parse::<usize>().unwrap();

        calibrations.push(calibration);
    }

    calibrations
}

pub fn load_file(demo: bool) -> io::Lines<io::BufReader<File>> {
    let filename = match demo {
        true => "src/bin/day01/part01/input-demo.txt",
        false => "src/bin/day01/part01/input.txt",
    };

    let file = File::open(filename).expect("Could not load input file");
    io::BufReader::new(file).lines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_result_ok() {
        let lines = load_file(true);
        let total = resolve_total_calibrations(lines);
        assert_eq!(total, 142);
    }

    #[test]
    fn real_result_ok() {
        let lines = load_file(false);
        let total = resolve_total_calibrations(lines);
        assert_eq!(total, 54632);
    }

    #[test]
    fn demo_calibrations_ok() {
        let lines = load_file(true);
        let calibrations = resolve_calibrations(lines);
        assert_eq!(calibrations, vec![12, 38, 15, 77]);
    }
}
