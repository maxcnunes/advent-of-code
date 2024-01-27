pub fn process() -> usize {
    let input = include_str!("./input.txt");

    get_lowest_location(input)
}

fn get_lowest_location(input: &str) -> usize {
    let (seeds, info_maps) = parse_garden_info(input);

    let mut is_first = true;
    let mut lowest_location = 0;

    // TODO: This logic is quite slow for large input sets.
    // Check if there is any possible performance improvements.
    for seed in seeds {
        let mut target = seed;

        for info_map in &info_maps {
            for (dst, src, len) in &info_map.infos {
                if target >= *src && target <= *src + len {
                    target = dst + (target - src);
                    break;
                }
            }
        }

        if is_first || target < lowest_location {
            is_first = false;
            lowest_location = target;
        }
    }

    lowest_location
}

#[derive(Debug, PartialEq)]
struct InfoMap {
    name: String,
    infos: Vec<(usize, usize, usize)>,
}

fn parse_garden_info(input: &str) -> (Vec<usize>, Vec<InfoMap>) {
    let mut seeds = vec![];
    let mut info_maps = vec![];

    let mut info_map = InfoMap {
        name: "".to_string(),
        infos: vec![],
    };

    for line in input.lines() {
        // Process first line which contains the seeds list
        if line.starts_with("seeds:") {
            let num_part = line
                .split(":")
                .nth(1)
                .expect("Could not get seeds numbers part");

            let mut seeds_infos = num_part
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap_or(0))
                .filter(|n| n > &0)
                .peekable();

            while seeds_infos.peek().is_some() {
                let start = seeds_infos.next().expect("Could not get seed start");
                let range = seeds_infos.next().expect("Could not get seed range");

                for seed in start..start + range {
                    seeds.push(seed);
                }
            }

            continue;
        }

        // Detected it will start on the next iteration to process a new info map section
        if line.trim() == "" {
            if info_map.name != "" {
                info_maps.push(info_map);
            }

            // Reset info map to start processing a new one on the next line
            info_map = InfoMap {
                name: "".to_string(),
                infos: vec![],
            };

            continue;
        }

        if info_map.name == "" {
            info_map.name = line.to_string().replace(":", "");
            continue;
        }

        let nums: Vec<usize> = line
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap_or(0))
            .collect();

        if nums.len() != 3 {
            panic!("Expected 3 numbers got {}", nums.len());
        }

        info_map.infos.push((nums[0], nums[1], nums[2]));
    }

    info_maps.push(info_map);

    (seeds, info_maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_DEMO: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn process_result() {
        assert_eq!(process(), 148041808);
    }

    #[test]
    fn demo_result() {
        let location = get_lowest_location(INPUT_DEMO);
        assert_eq!(location, 46);
    }

    #[test]
    fn demo_parse_garden_info() {
        let (seeds, info_maps) = parse_garden_info(INPUT_DEMO);
        assert_eq!(
            seeds,
            vec![
                // first seeds line
                79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92,
                // second seeds line
                55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67
            ]
        );
        assert_eq!(info_maps.len(), 7);
        assert_eq!(
            info_maps[0],
            InfoMap {
                name: "seed-to-soil map".to_string(),
                infos: vec![(50, 98, 2), (52, 50, 48)]
            }
        );
        assert_eq!(
            info_maps[1],
            InfoMap {
                name: "soil-to-fertilizer map".to_string(),
                infos: vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)]
            }
        );
        assert_eq!(
            info_maps[2],
            InfoMap {
                name: "fertilizer-to-water map".to_string(),
                infos: vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)]
            }
        );
        assert_eq!(
            info_maps[3],
            InfoMap {
                name: "water-to-light map".to_string(),
                infos: vec![(88, 18, 7), (18, 25, 70)]
            }
        );
        assert_eq!(
            info_maps[4],
            InfoMap {
                name: "light-to-temperature map".to_string(),
                infos: vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)]
            }
        );
        assert_eq!(
            info_maps[5],
            InfoMap {
                name: "temperature-to-humidity map".to_string(),
                infos: vec![(0, 69, 1), (1, 0, 69)]
            }
        );
        assert_eq!(
            info_maps[6],
            InfoMap {
                name: "humidity-to-location map".to_string(),
                infos: vec![(60, 56, 37), (56, 93, 4)]
            }
        );
    }
}
