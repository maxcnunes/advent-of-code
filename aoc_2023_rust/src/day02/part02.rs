use std::collections::HashMap;

pub fn process() -> usize {
    let input = include_str!("./input.txt");

    get_total_ids_possible_games(input)
}

fn get_total_ids_possible_games(input: &str) -> usize {
    let calibrations = get_ids_possible_games(input);

    calibrations.iter().fold(0, |acc, n| acc + n)
}

fn get_ids_possible_games(input: &str) -> Vec<usize> {
    let mut games: Vec<usize> = vec![];

    for line in input.lines() {
        let mut game_max_colors = HashMap::new();

        for gps in line.split(";") {
            for gp in gps.split(",") {
                for section in gp.split(":") {
                    let mut parts = section.split(" ");

                    let mut first = parts.next().expect("Could not get key part");
                    if first == "" {
                        first = parts.next().expect("Could not get key part");
                    }

                    if first == "Game" {
                        continue;
                    }

                    let second = parts.next().expect("Could not get value part");

                    let key = second;
                    let val = &first;

                    let num = val.parse::<usize>().expect("Could not parse number part");

                    let max_num = game_max_colors
                        .get(key)
                        .or(Some(&0))
                        .expect("Could not get num from hashmap");
                    if num > *max_num {
                        game_max_colors.insert(key, num);
                    }
                }
            }
        }

        let power = game_max_colors.values().fold(1, |acc, n| acc * n);

        games.push(power);
    }

    games
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_DEMO: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_process() {
        assert_eq!(process(), 72422);
    }

    #[test]
    fn demo_result_ok() {
        let total = get_total_ids_possible_games(INPUT_DEMO);
        assert_eq!(total, 2286);
    }

    #[test]
    fn demo_games() {
        let calibrations = get_ids_possible_games(INPUT_DEMO);
        assert_eq!(calibrations, vec![48, 12, 1560, 630, 36]);
    }
}
