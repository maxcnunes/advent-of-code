use super::part01::{parse_engine_parts, EnginePart};

pub fn process() -> usize {
    let input = include_str!("./input.txt");

    get_total_engine_part_numbers(input)
}

fn get_total_engine_part_numbers(input: &str) -> usize {
    let numbers = get_engine_part_numbers(input);

    numbers.iter().fold(0, |acc, n| acc + n)
}

fn get_engine_part_numbers(input: &str) -> Vec<usize> {
    let engine_part_lines = parse_engine_parts(input);

    let mut part_numbers = vec![];

    let line_len = input
        .lines()
        .next()
        .expect("Could not get first line")
        .len();

    for (line_index, line_parts) in engine_part_lines.iter().enumerate() {
        let curr_line = &engine_part_lines[line_index];

        for part in line_parts {
            if let Some(c) = part.symbol {
                if c != '*' {
                    continue;
                }

                let mut numbers: Vec<usize> = vec![];

                // check previous line
                if line_index > 0 {
                    let prev_line = &engine_part_lines[line_index - 1];
                    collect_numbers_close(&prev_line, &part, &line_len, &mut numbers);
                }

                // check current line
                collect_numbers_close(&curr_line, &part, &line_len, &mut numbers);

                // check next line
                if line_index < engine_part_lines.len() - 1 {
                    let next_line = &engine_part_lines[line_index + 1];
                    collect_numbers_close(&next_line, &part, &line_len, &mut numbers);
                }

                if numbers.len() > 1 {
                    let total = numbers.iter().fold(1, |acc, n| acc * n);
                    part_numbers.push(total);
                }
            }
        }
    }

    part_numbers
}

fn collect_numbers_close(
    line_parts: &Vec<EnginePart>,
    symbol_part: &EnginePart,
    line_len: &usize,
    numbers: &mut Vec<usize>,
) {
    // extend range beginning to match any close record
    let symbol_part_index_0 = if symbol_part.index_range.0 > 0 {
        symbol_part.index_range.0 - 1
    } else {
        symbol_part.index_range.0
    };

    // extend range ending to match any close record
    let symbol_part_index_1 = if symbol_part.index_range.1 < (line_len - 1) {
        symbol_part.index_range.1 + 1
    } else {
        symbol_part.index_range.1
    };

    for number_part in line_parts {
        if let Some(n) = number_part.number {
            let has_insersection_begin = symbol_part_index_0 <= number_part.index_range.0
                && symbol_part_index_1 >= number_part.index_range.0;

            let has_insersection_end = symbol_part_index_0 <= number_part.index_range.1
                && symbol_part_index_1 >= number_part.index_range.1;

            if has_insersection_begin || has_insersection_end {
                numbers.push(n);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_DEMO: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_process() {
        assert_eq!(process(), 84051670);
    }

    #[test]
    fn demo_result_ok() {
        let total = get_total_engine_part_numbers(INPUT_DEMO);
        assert_eq!(total, 467835);
    }
}
