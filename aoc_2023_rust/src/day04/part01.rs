use std::collections::HashMap;

pub fn process() -> usize {
    let input = include_str!("./input.txt");

    get_total_winnings(input)
}

fn get_total_winnings(input: &str) -> usize {
    let winnings = get_winnings(input);

    winnings
        .iter()
        .map(|nums| {
            nums.iter().fold(0, |acc, _n| {
                // The first match makes the card worth one point and each match
                // after the first doubles the point value of that card.
                if acc == 0 {
                    return 1;
                }
                acc * 2
            })
        })
        .fold(0, |acc, n| acc + n)
}

pub fn get_winnings(input: &str) -> Vec<Vec<usize>> {
    let mut cards_winnings = vec![];

    for line in input.lines() {
        let card = line.split(":").nth(1).expect("Could not get card data");

        let mut card_parts = card.split("|");

        let winnings_parts = card_parts.next().expect("Could not get winnings part");
        let numbers_parts = card_parts.next().expect("Could not get numbers part");

        let winnings = winnings_parts
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap_or(0))
            .filter(|n| n > &0);

        let numbers_hashmap: HashMap<usize, usize> = numbers_parts
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap_or(0))
            .filter(|n| n > &0)
            .map(|n| (n, n))
            .collect();

        let matching: Vec<usize> = winnings
            .filter(|w| numbers_hashmap.contains_key(w))
            .collect();

        cards_winnings.push(matching);
    }

    cards_winnings
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_DEMO: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_process() {
        assert_eq!(process(), 22193);
    }

    #[test]
    fn demo_result_ok() {
        let total = get_total_winnings(INPUT_DEMO);
        assert_eq!(total, 13);
    }

    #[test]
    fn demo_games() {
        let winnings = get_winnings(INPUT_DEMO);
        assert_eq!(
            winnings,
            vec![
                // card 1
                vec![48, 83, 86, 17],
                // card 2
                vec![32, 61],
                // card 3
                vec![1, 21],
                // card 4
                vec![84],
                // card 5
                vec![],
                // card 6
                vec![],
            ]
        );
    }

    #[test]
    fn demo_winnings_first_card() {
        let winnings = get_winnings("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(winnings, vec![vec![48, 83, 86, 17]]);
    }
}
