pub fn process() -> usize {
    let input = include_str!("./input.txt");

    get_lowest_location(input)
}

fn get_lowest_location(input: &str) -> usize {
    let mut locations = get_seeds_locations(input);
    locations.sort();
    locations[0]
}

fn get_seeds_locations(input: &str) -> Vec<usize> {
    let (seeds, info_maps) = parse_garden_info(input);

    let mut locations = vec![];

    for seed in seeds {
        // println!("Seed {seed}");

        let mut target = seed;

        for info_map in &info_maps {
            // println!("  Resolving {}", info_map.name);
            for (dst, src, len) in &info_map.infos {
                if target >= *src && target <= *src + len {
                    target = dst + (target - src);
                    // println!("    Found target {target} moving to the next map (dst={dst} src={src} len={len})");
                    break;
                }
            }
        }

        locations.push(target);
    }
    // println!("locations {:?}", locations);

    locations
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

            seeds = num_part
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap_or(0))
                .filter(|n| n > &0)
                .collect();

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
        assert_eq!(process(), 214922730);
    }

    #[test]
    fn demo_result() {
        let location = get_lowest_location(INPUT_DEMO);
        assert_eq!(location, 35);
    }

    #[test]
    fn demo_get_seeds_locations() {
        let locations = get_seeds_locations(INPUT_DEMO);

        // Expected locations:
        // * Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
        // * Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
        // * Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
        // * Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
        assert_eq!(locations, [82, 43, 86, 35]);
    }

    #[test]
    fn demo_parse_garden_info() {
        let (seeds, info_maps) = parse_garden_info(INPUT_DEMO);
        assert_eq!(seeds, vec![79, 14, 55, 13]);
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
