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
