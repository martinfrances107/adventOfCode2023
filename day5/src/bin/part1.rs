use core::ops::Range;
use core::str::Lines;
fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Debug, Default, PartialEq)]
struct SeedMapStage {
    range: Range<i128>,
    offset: i128,
}

#[derive(Debug, Default, PartialEq)]
struct SeedMap {
    pub stages: Vec<SeedMapStage>,
}

impl SeedMap {
    fn convert(&self, x: i128) -> i128 {
        for stage in &self.stages {
            if stage.range.contains(&x) {
                return x + stage.offset;
            }
        }
        x
    }
}
#[derive(Debug, Clone)]
struct ConverterErr;

impl TryFrom<&str> for SeedMapStage {
    type Error = ConverterErr;
    fn try_from(input: &str) -> Result<SeedMapStage, Self::Error> {
        let mut numbers = input.split(' ').filter_map(|x| x.parse::<i128>().ok());
        // TODO is there a singler way of rejecting the failure to parse as early as possible.
        let dst = numbers.next().ok_or(ConverterErr)?;
        let start = numbers.next().ok_or(ConverterErr)?;
        let range_len = numbers.next().ok_or(ConverterErr)?;
        dbg!(&dst);
        dbg!(&start);
        dbg!(&range_len);
        let end = start + range_len;
        let offset = dst - start;
        Ok(SeedMapStage {
            range: start..end,
            offset,
        })
    }
}
#[derive(Debug, Default, PartialEq)]
struct Almanac {
    pub maps: Vec<SeedMap>,
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        // let headerIterBlock = input.lines().take(2);
        let mut lines = input.lines();
        let h1 = lines.next();
        let h2 = lines.next();

        // assert than a title line is observed ( a line with a ':')

        let mut almanac = Almanac::default();
        //Looping over titled map definitions.
        'block_loop: loop {
            // At the start of a block
            // expect block heading
            let mut seedMap = SeedMap::default();

            match lines.next() {
                Some(line) => {
                    let have_title = line.contains(':');
                    if !have_title {
                        panic!("was expecting a title");
                    }
                }
                None => {
                    // eof
                    dbg!("breaking at end of file");
                    panic!("why here");
                }
            };

            loop {
                match lines.next() {
                    Some(line) => {
                        dbg!(&line);
                        match line.try_into() {
                            Ok(stage) => {
                                println!("pushing to block list");
                                dbg!(&stage);
                                seedMap.stages.push(stage);
                            }
                            Err(_) => {
                                // Something "not a valid converter" is treated as a blank line.
                                dbg!("was that a blank line");
                                almanac.maps.push(seedMap);
                                break 'block_loop;
                            }
                        };
                    }
                    None => {
                        dbg!("EOF pushing final block list");
                        almanac.maps.push(seedMap);
                        break 'block_loop;
                    }
                }
            }
        }
        almanac
    }
}

impl Almanac {
    fn location_from_seed(&self, seed: &i128) -> i128 {
        *seed
    }
}

fn part1(input: &str) -> u32 {
    let _al: Almanac = input.into();
    todo!();
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn converter() {
        let dataset: Vec<[i128; 2]> = vec![
            [0, 0],
            [1, 1],
            [48, 48],
            [49, 49],
            [50, 52],
            [51, 53],
            [96, 98],
            [97, 99],
            [98, 50],
            [99, 51],
        ];

        let input_str = r"H1

Title:
50 98 2
52 50 48";

        let almanac: Almanac = input_str.into();
        for [input, expected] in dataset {
            let actual = almanac.maps[0].convert(input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_blank_line_fails_to_convert() {
        let input = "";
        if let Ok(c) = SeedMapStage::try_from(input) {
            dbg!(c);
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    // Inspeciton of the struct of the Almanac.
    // Give the "converter" test this maybe hard of justify
    fn one_block() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48
";
        let actual: Almanac = input.into();

        let expected = Almanac {
            maps: vec![SeedMap {
                stages: vec![
                    SeedMapStage {
                        range: 98..100,
                        offset: -48,
                    },
                    SeedMapStage {
                        range: 50..98,
                        offset: 2,
                    },
                ],
            }],
        };

        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn example() {
        let input = r"seeds: 79 14 55 13

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
56 93 4";

        let seed_location_reference: Vec<[i128; 2]> = vec![[79, 82], [14, 43], [55, 86], [13, 35]];

        let almanac: Almanac = input.into();

        for [seed, location] in seed_location_reference {
            let computed_location = almanac.location_from_seed(&seed);
            assert_eq!(location, computed_location);
        }
    }
}
