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
            if let Some(num) = part.number {
                // check current line
                if has_symbol_close(&curr_line, &part, &line_len) {
                    part_numbers.push(num);
                    continue;
                }

                // check previous line
                if line_index > 0 {
                    let prev_line = &engine_part_lines[line_index - 1];
                    if has_symbol_close(&prev_line, &part, &line_len) {
                        part_numbers.push(num);
                        continue;
                    }
                }

                // check next line
                if line_index < engine_part_lines.len() - 1 {
                    let next_line = &engine_part_lines[line_index + 1];
                    if has_symbol_close(&next_line, &part, &line_len) {
                        part_numbers.push(num);
                        continue;
                    }
                }
            }
        }
    }

    part_numbers
}

fn has_symbol_close(
    line_parts: &Vec<EnginePart>,
    number_part: &EnginePart,
    line_len: &usize,
) -> bool {
    // extend range beginning to match any close record
    let number_part_index_0 = if number_part.index_range.0 > 0 {
        number_part.index_range.0 - 1
    } else {
        number_part.index_range.0
    };

    // extend range ending to match any close record
    let number_part_index_1 = if number_part.index_range.1 < (line_len - 1) {
        number_part.index_range.1 + 1
    } else {
        number_part.index_range.1
    };

    for symbol_part in line_parts {
        if symbol_part.symbol.is_some() {
            let has_insersection_begin = number_part_index_0 <= symbol_part.index_range.0
                && number_part_index_1 >= symbol_part.index_range.0;

            let has_insersection_end = number_part_index_0 <= symbol_part.index_range.1
                && number_part_index_1 >= symbol_part.index_range.1;

            if has_insersection_begin || has_insersection_end {
                return true;
            }
        }
    }

    false
}

pub fn parse_engine_parts(input: &str) -> Vec<Vec<EnginePart>> {
    let mut engine_parts: Vec<Vec<EnginePart>> = vec![];

    for line in input.lines() {
        let mut engine_parts_line: Vec<EnginePart> = vec![];

        let mut i = 0;
        let mut part = String::from("");
        let mut chars = line.chars().peekable();

        loop {
            let c = match chars.next() {
                Some(c) => c,
                None => break,
            };

            let next_c = chars.peek();
            // let last = next_c.is_none();

            // check part is complete
            if c != '.' {
                let completed = match next_c {
                    Some(nc) => c.is_digit(10) != nc.is_digit(10) || *nc == '.',
                    None => true,
                };

                if completed {
                    part = part + &c.to_string();
                    // println!("part complete={:?} ", part);

                    // println!("  i={:?} part.len={:?}", i, part.len());
                    let number = part.parse::<usize>().ok();
                    let symbol = match number {
                        Some(_) => None,
                        None => Some(part.chars().next().expect("Could not get symbol")),
                    };

                    engine_parts_line.push(EnginePart {
                        number,
                        symbol,
                        index_range: (i - (part.len() - 1), i),
                    });

                    part = "".to_string();
                } else {
                    part = part + &c.to_string();
                    // println!("part continue={:?}", part);
                }
            }

            i += 1;
        }

        engine_parts.push(engine_parts_line);
    }

    engine_parts
}

#[derive(Debug, PartialEq)]
pub struct EnginePart {
    pub number: Option<usize>,
    pub symbol: Option<char>,
    pub index_range: (usize, usize),
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
        assert_eq!(process(), 532428);
    }

    #[test]
    fn demo_result_ok() {
        let total = get_total_engine_part_numbers(INPUT_DEMO);
        assert_eq!(total, 4361);
    }

    #[test]
    fn demo_get_engine_part_numbers() {
        let numbers = get_engine_part_numbers(INPUT_DEMO);
        assert_eq!(numbers, vec![467, 35, 633, 617, 592, 755, 664, 598]);
    }

    #[test]
    fn demo_parse_engine_parts() {
        // 0123456789
        // ----------
        // 467..114..
        // ...*......
        // ..35..633.
        // ......#...
        // 617*......
        // .....+.58.
        // ..592.....
        // ......755.
        // ...$.*....
        // .664.598..
        let engine_parts = parse_engine_parts(INPUT_DEMO);
        assert_eq!(engine_parts.len(), 10);

        // line 1
        assert_eq!(
            engine_parts[0],
            vec![
                EnginePart {
                    number: Some(467),
                    symbol: None,
                    index_range: (0, 2)
                },
                EnginePart {
                    number: Some(114),
                    symbol: None,
                    index_range: (5, 7)
                }
            ],
        );

        // line 2
        assert_eq!(
            engine_parts[1],
            vec![EnginePart {
                number: None,
                symbol: Some('*'),
                index_range: (3, 3)
            },],
        );

        // line 3
        assert_eq!(
            engine_parts[2],
            vec![
                EnginePart {
                    number: Some(35),
                    symbol: None,
                    index_range: (2, 3)
                },
                EnginePart {
                    number: Some(633),
                    symbol: None,
                    index_range: (6, 8)
                }
            ],
        );
    }
}
