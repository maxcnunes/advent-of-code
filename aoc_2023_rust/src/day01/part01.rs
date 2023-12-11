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
        let first_num = line
            .chars()
            .find_map(|c| c.to_digit(10))
            .expect("could not find first calibration value");

        // Get last num
        let last_num = line
            .chars()
            .rev()
            .find_map(|c| c.to_digit(10))
            .expect("could not find last calibration value");

        let calibration_str = first_num.to_string() + &last_num.to_string();

        let calibration = calibration_str
            .parse::<usize>()
            .expect("could not parse value");

        calibrations.push(calibration);
    }

    calibrations
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_DEMO: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    #[test]
    fn test_process() {
        assert_eq!(process(), 54632);
    }

    #[test]
    fn demo_result_ok() {
        let total = resolve_total_calibrations(INPUT_DEMO);
        assert_eq!(total, 142);
    }

    #[test]
    fn demo_calibrations_ok() {
        let calibrations = resolve_calibrations(INPUT_DEMO);
        assert_eq!(calibrations, vec![12, 38, 15, 77]);
    }
}
