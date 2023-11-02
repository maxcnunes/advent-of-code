use std::collections::VecDeque;
use std::fs::File;
use std::io::{self};

use crate::error::SolutionError;
use crate::icon;

#[derive(Debug)]
struct MoveInstruction {
    src: usize,
    dst: usize,
    amount: usize,
}

// After the rearrangement procedure completes, what crate ends up on top of each stack?
pub fn process(lines: io::Lines<io::BufReader<File>>) -> std::result::Result<(), SolutionError> {
    let top_stacks = resolve_top_stacks(lines)?;

    println!(
        "{} Top stacks after rearrangement: {}",
        icon::CHECK_MARK,
        top_stacks
    );

    Ok(())
}

fn resolve_top_stacks(
    lines: io::Lines<io::BufReader<File>>,
) -> std::result::Result<String, SolutionError> {
    let data = load_data(lines)?;

    let mut stacks = data.0;
    let instructions = data.1;

    // println!("stacks {:?}", stacks);
    // println!("instructions {:?}", instructions);
    rearrenge_stacks(&mut stacks, &instructions)?;

    let top_stacks = get_top_stacks(&stacks);

    Ok(top_stacks)
}

fn load_data(
    lines: io::Lines<io::BufReader<File>>,
) -> Result<(Vec<VecDeque<char>>, Vec<MoveInstruction>), SolutionError> {
    let mut stacks: Vec<VecDeque<char>> = vec![];
    let mut instructions: Vec<MoveInstruction> = vec![];

    for line_result in lines {
        let line = line_result.map_err(SolutionError::GetLineErr)?;
        if line.starts_with("move") {
            instructions.push(parse_instruction(line));
        } else if line.contains("[") {
            load_stacks(&mut stacks, line);
        }
    }

    Ok((stacks, instructions))
}

fn rearrenge_stacks(
    stacks: &mut Vec<VecDeque<char>>,
    instructions: &Vec<MoveInstruction>,
) -> Result<(), SolutionError> {
    for inst in instructions {
        for _ in 1..=inst.amount {
            // println!("Move {} from {} to {}", inst.amount, inst.src, inst.dst);
            let c = stacks[inst.src - 1].pop_back();
            stacks[inst.dst - 1].push_back(c.unwrap());
        }
    }
    Ok(())
}

fn get_top_stacks(stacks: &Vec<VecDeque<char>>) -> String {
    let tops = stacks.iter().fold("".to_string(), |acc, stack| {
        acc + &stack.back().unwrap().to_string()
    });

    tops.to_string()
}

// Load stacks from this format below.
// Example:
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3
fn load_stacks(stacks: &mut Vec<VecDeque<char>>, line: String) {
    let mut col = 0;
    let mut next_at = 1;

    for (i, c) in line.chars().enumerate() {
        if i != next_at {
            continue;
        }

        next_at += 4;

        if stacks.len() < col + 1 {
            stacks.push(VecDeque::new());
        }

        if c != ' ' {
            stacks[col].push_front(c);
        }

        col += 1;
    }
}

// Load instructions from this format below.
// Example:
// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2
fn parse_instruction(line: String) -> MoveInstruction {
    let parts: Vec<usize> = line
        .split(" ")
        .map(|part| part.parse::<usize>().unwrap_or(0))
        .filter(|n| n.gt(&0))
        .collect();

    if parts.len() != 3 {
        panic!("wrong instruction format")
    }

    MoveInstruction {
        amount: parts[0],
        src: parts[1],
        dst: parts[2],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SolutionError;
    use crate::input;

    #[test]
    fn demo_result_ok() -> Result<(), SolutionError> {
        // Expected results:
        // in this example,
        // the top crates are C in stack 1,
        // M in stack 2,
        // and Z in stack 3,
        // so you should combine these together and give the Elves the message CMZ.
        let lines = input::load(5, true)?;
        let top_stacks = resolve_top_stacks(lines)?;
        assert_eq!(top_stacks, "CMZ");
        Ok(())
    }

    #[test]
    fn real_result_ok() -> Result<(), SolutionError> {
        let lines = input::load(5, false)?;
        let top_stacks = resolve_top_stacks(lines)?;
        assert_eq!(top_stacks, "SBPQRSCDF");
        Ok(())
    }

    #[test]
    fn test_load_stacks() {
        // Test loading this stack:
        //     [D]
        // [N] [C]
        // [Z] [M] [P]
        //  1   2   3
        let mut stacks: Vec<VecDeque<char>> = vec![];

        load_stacks(&mut stacks, "    [D]    ".to_string());
        assert_eq!(
            stacks,
            vec![
                VecDeque::from([]),    // stack 1
                VecDeque::from(['D']), // stack 2
                VecDeque::from([])     // stack 3
            ]
        );

        load_stacks(&mut stacks, "[N] [C]    ".to_string());
        assert_eq!(
            stacks,
            vec![
                VecDeque::from(['N']),      // stack 1
                VecDeque::from(['C', 'D']), // stack 2
                VecDeque::from([])          // stack 3
            ]
        );

        load_stacks(&mut stacks, "[Z] [M] [P]".to_string());
        assert_eq!(
            stacks,
            vec![
                VecDeque::from(['Z', 'N']),      // stack 1
                VecDeque::from(['M', 'C', 'D']), // stack 2
                VecDeque::from(['P'])            // stack 3
            ]
        );
    }

    #[test]
    fn test_parse_instruction() {
        // Test loading this instructions:
        // move 1 from 2 to 1
        // move 3 from 1 to 3
        // move 2 from 2 to 1
        // move 1 from 1 to 2
        let instruction = parse_instruction("move 1 from 2 to 1".to_string());
        assert_eq!(instruction.amount, 1);
        assert_eq!(instruction.src, 2);
        assert_eq!(instruction.dst, 1);

        let instruction = parse_instruction("move 3 from 1 to 3".to_string());
        assert_eq!(instruction.amount, 3);
        assert_eq!(instruction.src, 1);
        assert_eq!(instruction.dst, 3);

        let instruction = parse_instruction("move 2 from 2 to 1".to_string());
        assert_eq!(instruction.amount, 2);
        assert_eq!(instruction.src, 2);
        assert_eq!(instruction.dst, 1);

        let instruction = parse_instruction("move 1 from 1 to 2".to_string());
        assert_eq!(instruction.amount, 1);
        assert_eq!(instruction.src, 1);
        assert_eq!(instruction.dst, 2);
    }

    #[test]
    fn test_rearrenge_stacks() {
        let mut stacks = vec![
            VecDeque::from(['Z', 'N']),      // stack 1
            VecDeque::from(['M', 'C', 'D']), // stack 2
            VecDeque::from(['P']),           // stack 3
        ];

        let instructions = vec![
            MoveInstruction {
                amount: 1,
                src: 2,
                dst: 1,
            },
            MoveInstruction {
                amount: 3,
                src: 1,
                dst: 3,
            },
            MoveInstruction {
                amount: 2,
                src: 2,
                dst: 1,
            },
            MoveInstruction {
                amount: 1,
                src: 1,
                dst: 2,
            },
        ];

        let _ = rearrenge_stacks(&mut stacks, &instructions);
        assert_eq!(
            stacks,
            vec![
                vec!['C'],                // stack 1
                vec!['M'],                // stack 2
                vec!['P', 'D', 'N', 'Z']  // stack 3
            ]
        );
    }

    #[test]
    fn test_get_top_stacks() {
        let stacks = vec![
            VecDeque::from(['C']),                // stack 1
            VecDeque::from(['M']),                // stack 2
            VecDeque::from(['P', 'D', 'N', 'Z']), // stack 3
        ];

        assert_eq!(get_top_stacks(&stacks), "CMZ");
    }
}
