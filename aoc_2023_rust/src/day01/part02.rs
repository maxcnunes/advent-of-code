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
        let mut first_num: Option<u32> = None;
        for c in line.chars() {
            first_num = c.to_digit(10);
            if first_num.is_some() {
                break;
            }

            text_chunk_left.push(c);

            first_num = get_word_value_from_text(&text_chunk_left);
            if first_num.is_some() {
                break;
            }
        }

        // Get last num
        let mut text_chunk_right = "".to_string();
        let mut last_num: Option<u32> = None;
        for c in line.chars().rev() {
            last_num = c.to_digit(10);
            if last_num.is_some() {
                break;
            }

            text_chunk_right.insert(0, c);

            last_num = get_word_value_from_text(&text_chunk_right);
            if last_num.is_some() {
                break;
            }
        }

        let calibration_str = first_num.expect("missing first value").to_string()
            + &last_num.expect("missing last value").to_string();

        let calibration = calibration_str.parse::<usize>().unwrap();

        calibrations.push(calibration);
    }

    calibrations
}

fn get_word_value_from_text(text: &String) -> Option<u32> {
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    words.iter().find_map(|&w| -> Option<u32> {
        if !text.contains(&w) {
            return None;
        }

        let num = match w {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!("Invalid word: {w}"),
        };

        Some(num)
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
