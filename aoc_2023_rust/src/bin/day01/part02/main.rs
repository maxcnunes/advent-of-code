use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = load_file(false);

    let total = resolve_total_calibrations(lines);

    println!(
        "Day 01, Part 02: The sum of all of the calibration values: {}",
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
        let mut text_chunk_left = "".to_string();
        let mut first_num: Option<String> = None;
        for c in line.chars() {
            text_chunk_left.push(c);

            if let Some(num) = get_word_value_from_text(&text_chunk_left) {
                first_num = Some(num);
                break;
            }
        }

        // Get last num
        let mut text_chunk_right = "".to_string();
        let mut last_num: Option<String> = None;
        for c in line.chars().rev() {
            text_chunk_right.insert(0, c);

            if let Some(num) = get_word_value_from_text(&text_chunk_right) {
                last_num = Some(num);
                break;
            }
        }

        let calibration_str =
            first_num.expect("missing first value") + &last_num.expect("missing last value");

        let calibration = calibration_str.parse::<usize>().unwrap();

        calibrations.push(calibration);
    }

    calibrations
}

fn get_word_value_from_text(text: &String) -> Option<String> {
    let words = vec![
        "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven",
        "8", "eight", "9", "nine",
    ];

    words.iter().find_map(|&w| -> Option<String> {
        if !text.contains(&w) {
            return None;
        }

        match w {
            "1" | "one" => Some("1".to_string()),
            "2" | "two" => Some("2".to_string()),
            "3" | "three" => Some("3".to_string()),
            "4" | "four" => Some("4".to_string()),
            "5" | "five" => Some("5".to_string()),
            "6" | "six" => Some("6".to_string()),
            "7" | "seven" => Some("7".to_string()),
            "8" | "eight" => Some("8".to_string()),
            "9" | "nine" => Some("9".to_string()),
            _ => panic!("Invalid word: {w}"),
        }
    })
}

pub fn load_file(demo: bool) -> io::Lines<io::BufReader<File>> {
    let filename = match demo {
        true => "src/bin/day01/part02/input-demo.txt",
        false => "src/bin/day01/part02/input.txt",
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
        assert_eq!(total, 281);
    }

    #[test]
    fn real_result_ok() {
        let lines = load_file(false);
        let total = resolve_total_calibrations(lines);
        assert_eq!(total, 54019);
    }

    #[test]
    fn demo_calibrations_ok() {
        let lines = load_file(true);
        let calibrations = resolve_calibrations(lines);
        assert_eq!(calibrations, vec![29, 83, 13, 24, 42, 14, 76]);
    }
}
