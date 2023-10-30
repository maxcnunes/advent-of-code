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

        let parts = line
            .split_once(",")
            .ok_or(SolutionError::CouldNotSplitByCommaErr)?;

        let a = parse_section(parts.0)?;
        let b = parse_section(parts.1)?;

        let ok = is_overlapping(&a, &b);
        if ok {
            total += 1;
        }
    }

    Ok(total)
}

#[derive(Debug)]
struct Section {
    start: u32,
    end: u32,
}

// check if there is any overlapping between two sections
fn is_overlapping(a: &Section, b: &Section) -> bool {
    // a and b have either the same start or end values
    if a.start == b.start || a.end == b.end || a.start == b.end || b.start == a.end {
        return true;
    }

    // b within a
    if a.start <= b.start && a.end >= b.end {
        return true;
    }

    // a within b
    if b.start <= a.start && b.end >= a.end {
        return true;
    }

    // overlapping at the beginning
    if a.start <= b.start && a.end <= b.end && a.end >= b.start {
        return true;
    }

    // overlapping at the ending
    if b.start <= a.start && b.end <= a.end && b.end >= a.start {
        return true;
    }

    // no overlapping
    false
}

fn parse_section(part: &str) -> Result<Section, SolutionError> {
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

    Ok(Section { start, end })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SolutionError;
    use crate::input;

    #[test]
    fn demo_result_ok() -> Result<(), SolutionError> {
        // Expected results:
        // * 5-7,7-9 overlaps in a single section, 7.
        // * 2-8,3-7 overlaps all of the sections 3 through 7.
        // * 6-6,4-6 overlaps in a single section, 6.
        // * 2-6,4-8 overlaps in sections 4, 5, and 6.
        // So, in this example, the number of overlapping assignment pairs is 4.
        let lines = input::load(4, true)?;
        let total = calc_total_overlaps(lines)?;
        assert_eq!(total, 4);
        Ok(())
    }

    #[test]
    fn real_result_ok() -> Result<(), SolutionError> {
        let lines = input::load(4, false)?;
        let total = calc_total_overlaps(lines)?;
        assert_eq!(total, 801);
        Ok(())
    }

    #[test]
    fn is_overlapping_true() {
        // all tests swipe a and b to test both sections in different position
        let test_cases = vec![
            // equal
            (Section { start: 1, end: 1 }, Section { start: 1, end: 1 }),
            // one section start is equal to the other section end
            (Section { start: 4, end: 5 }, Section { start: 2, end: 4 }),
            // one section start is equal to the other section start
            (Section { start: 1, end: 2 }, Section { start: 1, end: 3 }),
            // one section end is equal to the other section end
            (Section { start: 2, end: 4 }, Section { start: 3, end: 4 }),
            // one section is within the other
            (Section { start: 2, end: 8 }, Section { start: 3, end: 7 }),
            // demo input expected to be valid
            (Section { start: 5, end: 7 }, Section { start: 7, end: 9 }),
            (Section { start: 2, end: 8 }, Section { start: 3, end: 7 }),
            (Section { start: 6, end: 6 }, Section { start: 4, end: 6 }),
            (Section { start: 2, end: 6 }, Section { start: 4, end: 8 }),
            // a few more cases
            (Section { start: 4, end: 6 }, Section { start: 5, end: 7 }),
            (Section { start: 4, end: 6 }, Section { start: 5, end: 6 }),
            (Section { start: 4, end: 6 }, Section { start: 5, end: 5 }),
            (Section { start: 4, end: 5 }, Section { start: 5, end: 5 }),
            (Section { start: 3, end: 8 }, Section { start: 2, end: 5 }),
        ];

        for tc in test_cases {
            assert_eq!(
                is_overlapping(&tc.0, &tc.1),
                true,
                "a={:?} b={:?}",
                tc.0,
                tc.1
            );

            assert_eq!(
                is_overlapping(&tc.1, &tc.0),
                true,
                "a={:?} b={:?}",
                tc.1,
                tc.0
            );
        }
    }

    #[test]
    fn is_overlapping_false() {
        // all tests swipe a and b to test both sections in different position
        let test_cases = vec![
            // equal
            (Section { start: 0, end: 0 }, Section { start: 1, end: 1 }),
            (Section { start: 1, end: 1 }, Section { start: 2, end: 2 }),
            (Section { start: 1, end: 2 }, Section { start: 3, end: 4 }),
        ];

        for tc in test_cases {
            assert_eq!(
                is_overlapping(&tc.0, &tc.1),
                false,
                "a={:?} b={:?}",
                tc.0,
                tc.1
            );

            assert_eq!(
                is_overlapping(&tc.1, &tc.0),
                false,
                "a={:?} b={:?}",
                tc.1,
                tc.0
            );
        }
    }
}
