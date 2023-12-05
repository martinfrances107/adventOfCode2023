use core::ops::Range;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Debug, Default, PartialEq)]
struct SubRange {
    range: Range<i128>,
    offset: i128,
}

#[derive(Debug, Default, PartialEq)]
struct SeedMapStage {
    subs: Vec<SubRange>,
}

impl SeedMapStage {
    fn convert(&self, x: i128) -> i128 {
        for sub in &self.subs {
            if sub.range.contains(&x) {
                return x + sub.offset;
            }
        }
        x
    }
}
#[derive(Debug, Clone)]
struct ConverterErr;

impl TryFrom<&str> for SubRange {
    type Error = ConverterErr;
    fn try_from(input: &str) -> Result<SubRange, Self::Error> {
        let mut numbers = input.split(' ').filter_map(|x| x.parse::<i128>().ok());
        // TODO is there a singler way of rejecting the failure to parse as early as possible.
        let dst = numbers.next().ok_or(ConverterErr)?;
        let start = numbers.next().ok_or(ConverterErr)?;
        let range_len = numbers.next().ok_or(ConverterErr)?;
        let end = start + range_len;
        let offset = dst - start;
        Ok(SubRange {
            range: start..end,
            offset,
        })
    }
}

#[derive(Debug, Default, PartialEq)]
struct SeedMap {
    pub stages: Vec<SeedMapStage>,
}

#[derive(Debug, Default, PartialEq)]
struct Almanac {
    pub seeds: Vec<i128>,
    pub stages: Vec<SeedMapStage>,
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let mut almanac = Almanac::default();

        let mut lines = input.lines();
        let seed_line = lines.next().expect("could not read the seed line");
        let Some((_header, seed_number_line)) = seed_line.split_once(':') else {
            panic!("no seed line");
        };
        let seeds = seed_number_line
            .split(' ')
            .filter_map(|x| x.parse::<i128>().ok())
            .collect::<Vec<_>>();
        println!("{:#?}", seeds);
        almanac.seeds = seeds;
        let _blank = lines.next();

        // assert than a title line is observed ( a line with a ':')

        //Looping over titled map definitions.
        'block_loop: loop {
            // At the start of a block
            // expect block heading

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
            let mut subs: Vec<SubRange> = vec![];
            loop {
                match lines.next() {
                    Some(line) => {
                        dbg!(&line);
                        match line.try_into() {
                            Ok(subrange) => {
                                // println!("pushing subrange");
                                // dbg!(&subrange);
                                subs.push(subrange);
                            }
                            Err(_) => {
                                // Something "not a valid converter" is treated as a blank line.
                                // dbg!("pushing subs");
                                // dbg!(&subs);
                                let stage = SeedMapStage { subs };
                                almanac.stages.push(stage);
                                break;
                            }
                        };
                    }
                    None => {
                        dbg!("EOF pushing final block list");
                        let stage = SeedMapStage { subs };
                        almanac.stages.push(stage);
                        break 'block_loop;
                    }
                }
            }
        }
        almanac
    }
}

impl Almanac {
    // using the seed number as initial value
    // "stream" as in pass the value from the one stage into the input of the next.
    fn stream(&self, seed: i128) -> i128 {
        dbg!(&self.stages);
        println!("streaming .................................");
        let mut value = seed;
        dbg!(&value);
        for stage in self.stages.iter() {
            value = stage.convert(value);
            dbg!(&value);
        }
        value
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

        let input_str = r"seeds: 79 14 55 13

Title:
50 98 2
52 50 48";

        let almanac: Almanac = input_str.into();
        dbg!(&almanac);
        for [input, expected] in dataset {
            let actual = almanac.stages[0].convert(input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_blank_line_fails_to_convert() {
        let input = "";

        if let Ok(subrange) = SubRange::try_from(input) {
            dbg!(subrange);
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
            seeds: vec![79, 14, 55, 13],
            stages: vec![SeedMapStage {
                subs: vec![
                    SubRange {
                        range: 98..100,
                        offset: -48,
                    },
                    SubRange {
                        range: 50..98,
                        offset: 2,
                    },
                ],
            }],
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn soil() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48
";

        let a: Almanac = input.into();

        // maps seed to fertilizer
        let dataset: Vec<[i128; 2]> = vec![[79, 81], [14, 14], [55, 57], [13, 13]];
        for [seed, expected] in dataset {
            let actual = a.stream(seed);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn fertilizer() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15
";

        let a: Almanac = input.into();

        // maps seed to fertilizer
        let dataset: Vec<[i128; 2]> = vec![[79, 81], [14, 53], [55, 57], [13, 52]];
        for [seed, expected] in dataset {
            let actual = a.stream(seed);
            assert_eq!(actual, expected);
        }
    }

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
        dbg!(&almanac);
        for [seed, location] in seed_location_reference {
            let computed_location = almanac.stream(seed);
            assert_eq!(location, computed_location);
        }
    }
}
