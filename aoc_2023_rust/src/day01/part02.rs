pub fn process() -> usize {
    let input = include_str!("./input.txt");

    resolve_total_calibrations(input)
}

fn resolve_total_calibrations(input: &str) -> usize {
    let calibrations = resolve_calibrations(input);

    calibrations.iter().fold(0, |acc, n| acc + n)
}

fn resolve_calibrations(input: &str) -> Vec<usize> {
    let mut calibrations: Vec<usize> = vec![];

    for line in input.lines() {
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_DEMO: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn test_process() {
        assert_eq!(process(), 54019);
    }

    #[test]
    fn demo_result_ok() {
        let total = resolve_total_calibrations(INPUT_DEMO);
        assert_eq!(total, 281);
    }

    #[test]
    fn demo_calibrations_ok() {
        let calibrations = resolve_calibrations(INPUT_DEMO);
        assert_eq!(calibrations, vec![29, 83, 13, 24, 42, 14, 76]);
    }
}
