use crate::day04::part01::get_winnings;
use std::collections::HashMap;

pub fn process() -> usize {
    let input = include_str!("./input.txt");

    get_total_winnings(input)
}

fn get_total_winnings(input: &str) -> usize {
    let winnings = get_winnings(input);

    let cards = dup_cards(&winnings);

    cards.iter().fold(0, |acc, (_k, v)| acc + v)
}

fn dup_cards(winnings: &Vec<Vec<usize>>) -> HashMap<usize, usize> {
    let mut cards: HashMap<usize, usize> = winnings
        .iter()
        .enumerate()
        .map(|(i, _w)| (i + 1, 1))
        .collect();

    for (i, matches) in winnings.iter().enumerate() {
        let n_matches = matches.len();
        let card_id = i + 1;
        let instances = cards
            .get(&card_id)
            .expect("Could not get instances")
            .clone();

        let next_card_id = card_id + 1;
        let end_match = &card_id + n_matches;

        for line in next_card_id..=end_match {
            cards.entry(line).and_modify(|n| *n += instances);
        }
    }

    cards
}

fn pretty_cards(cards: &HashMap<usize, usize>) -> Vec<String> {
    let mut print_data = cards
        .iter()
        .map(|(k, v)| format!("card_{k}={v}"))
        .collect::<Vec<String>>();
    print_data.sort();
    print_data
}

#[cfg(test)]
mod tests {
    use crate::day04::part01::get_winnings;

    use super::*;

    const INPUT_DEMO: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_process() {
        // 18061 is not valid, too low
        assert_eq!(process(), 5625994);
    }

    #[test]
    fn demo_result_ok() {
        let total = get_total_winnings(INPUT_DEMO);
        assert_eq!(total, 30);
    }

    #[test]
    fn demo_dup_cards() {
        // - Card 1 has four matching numbers, so you win one copy each of the next four cards: cards 2, 3, 4, and 5.
        // - Your original card 2 has two matching numbers, so you win one copy each of cards 3 and 4.
        // - Your copy of card 2 also wins one copy each of cards 3 and 4.
        // - Your four instances of card 3 (one original and three copies) have two matching numbers, so you win four copies each of cards 4 and 5.
        // - Your eight instances of card 4 (one original and seven copies) have one matching number, so you win eight copies of card 5.
        // - Your fourteen instances of card 5 (one original and thirteen copies) have no matching numbers and win no more cards.
        // - Your one instance of card 6 (one original) has no matching numbers and wins no more cards.
        //
        // Once all of the originals and copies have been processed, you end up with 1 instance of card 1, 2 instances of card 2, 4 instances of card 3, 8 instances of card 4, 14 instances of card 5, and 1 instance of card 6. In total, this example pile of scratchcards causes you to ultimately have 30 scratchcards!
        //
        // Card 1: 01 instance
        // Card 2: 02 instances
        // Card 3: 04 instances
        // Card 4: 08 instances
        // Card 5: 14 instances
        // Card 6: 01 instance
        let winnings = get_winnings(INPUT_DEMO);
        let cards = dup_cards(&winnings);
        let pretty = pretty_cards(&cards);
        assert_eq!(
            pretty,
            vec![
                "card_1=1",
                "card_2=2",
                "card_3=4",
                "card_4=8",
                "card_5=14",
                "card_6=1"
            ]
        );
    }

    #[test]
    fn demo_winnings_first_card() {
        let winnings = get_winnings("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(winnings, vec![vec![48, 83, 86, 17]]);
    }
}
