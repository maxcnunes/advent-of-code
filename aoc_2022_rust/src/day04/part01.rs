use std::fs::File;
use std::io::{self};

use crate::error::SolutionError;
use crate::icon;

// In how many assignment pairs does one range fully contain the other?
pub fn process(lines: io::Lines<io::BufReader<File>>) -> std::result::Result<(), SolutionError> {
    let total = calc_total_overlaps(lines)?;

    println!("{} Total overlaps: {}", icon::CHECK_MARK, total);

    Ok(())
}

fn calc_total_overlaps(lines: io::Lines<io::BufReader<File>>) -> Result<usize, SolutionError> {
    let mut total: usize = 0;

    for line_result in lines {
        let line = line_result.map_err(SolutionError::GetLineErr)?;

        let ok = is_overlapping(line)?;
        if ok {
            total += 1;
        }
    }

    Ok(total)
}

fn is_overlapping(line: String) -> Result<bool, SolutionError> {
    let parts = line
        .split_once(",")
        .ok_or(SolutionError::CouldNotSplitByCommaErr)?;

    let a = parse_section(parts.0)?;
    let b = parse_section(parts.1)?;

    // Is either a or b overlapping each other?
    Ok((a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1))
}

fn parse_section(part: &str) -> Result<(u32, u32), SolutionError> {
    let raw_sec = part
        .split_once("-")
        .ok_or(SolutionError::CouldNotSplitByDashErr)?;

    let start = raw_sec
        .0
        .parse()
        .map_err(SolutionError::CouldNotParseSectionStartValueErr)?;
    let end = raw_sec
        .1
        .parse()
        .map_err(SolutionError::CouldNotParseSectionEndValueErr)?;

    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SolutionError;
    use crate::input;

    #[test]
    fn demo_result_ok() -> Result<(), SolutionError> {
        // Expected results:
        // 2-8 fully contains 3-7
        // 6-6 is fully contained by 4-6
        // Those are the 2 overlaps in the list.
        let lines = input::load(4, true)?;
        let total = calc_total_overlaps(lines)?;
        assert_eq!(total, 2);
        Ok(())
    }

    #[test]
    fn real_result_ok() -> Result<(), SolutionError> {
        let lines = input::load(4, false)?;
        let total = calc_total_overlaps(lines)?;
        assert_eq!(total, 444);
        Ok(())
    }
}
