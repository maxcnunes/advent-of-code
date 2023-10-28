use std::fs::File;
use std::io::{self};

use crate::error::SolutionError;
use crate::icon;

pub fn process(lines: io::Lines<io::BufReader<File>>) -> std::result::Result<(), SolutionError> {
    let total = cal_priorities(lines)?;

    // Find the item type that appears in both compartments of each rucksack.
    // What is the sum of the priorities of those item types?
    println!("{} Total number of priorities: {}", icon::CHECK_MARK, total);

    Ok(())
}

fn cal_priorities(lines: io::Lines<io::BufReader<File>>) -> Result<usize, SolutionError> {
    let mut total: usize = 0;

    let mut i = 0;
    for line_result in lines {
        i += 1;
        let line = line_result.map_err(SolutionError::GetLineErr)?;
        // println!("Line {}", line);

        total += get_backpack_priority(line, i)?;
    }

    Ok(total)
}

fn get_backpack_priority(line: String, line_num: usize) -> Result<usize, SolutionError> {
    let alphabet: Vec<_> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let items: Vec<_> = line.chars().collect();
    // println!("items {:?}", items);

    let size = items.len();
    let half = size / 2;

    for i_f in 1..=half {
        let f = items[i_f - 1];
        let f_low = f.to_lowercase().to_string();
        // println!("item {}({})", f, i_f);
        for i_s in half + 1..=size {
            let s = items[i_s - 1];
            // println!("  subitem {}({})", s, i_s);
            if f == s {
                let alphatbet_index_result = alphabet.iter().position(|c| c.to_string() == f_low);

                let alphatbet_index =
                    alphatbet_index_result.ok_or(SolutionError::CharNotFoundErr(s))?;

                let result = if f.is_lowercase() {
                    alphatbet_index + 1
                } else {
                    alphatbet_index + 27
                };

                println!(
                    "Found match line {} char {} alphatbet_index {} result {}",
                    line_num, f, alphatbet_index, result,
                );
                return Ok(result);
            }
        }
    }

    Err(SolutionError::CouldNotFindPriorityErr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SolutionError;
    use crate::input;

    #[test]
    fn demo_result_ok() -> Result<(), SolutionError> {
        // Expected results:
        // 1. 16 (p)
        // 2. 38 (L)
        // 3. 42 (P)
        // 4. 22 (v)
        // 5. 20 (t)
        // 6. 19 (s)
        // the sum of these is 157.
        let lines = input::load(3, true)?;
        let total = cal_priorities(lines)?;
        assert_eq!(total, 157);
        Ok(())
    }
}
