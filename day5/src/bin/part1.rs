use core::ops::Range;
use core::str::Lines;
fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Default, Debug)]
struct Converter {
    range_and_offset: Vec<(Range<i128>, i128)>,
}

impl Converter {
    fn convert(&self, x: i128) -> i128 {
        for (range, offset) in &self.range_and_offset {
            if range.contains(&x) {
                return x + *offset;
            }
        }
        x
    }
}
#[derive(Debug, Clone)]
struct ConverterErr;

impl TryFrom<&str> for Converter {
    type Error = ConverterErr;
    fn try_from(input: &str) -> Result<Converter, Self::Error> {
        let mut range_and_offset = vec![];
        for line in input.lines() {
            let mut inputs = line.split(' ').filter_map(|x| x.parse::<i128>().ok());
            // TODO is there a singler way of rejecting the failure to parse as early as possible.
            let dst = inputs.next().ok_or(ConverterErr)?;
            let start = inputs.next().ok_or(ConverterErr)?;
            let range_len = inputs.next().ok_or(ConverterErr)?;
            dbg!(&dst);
            dbg!(&start);
            dbg!(&range_len);
            let end = start + range_len;
            let offset = dst - start;
            range_and_offset.push((start..end, offset));
        }

        // If preseted with a blank line this will error.
        if range_and_offset.is_empty() {
            return Err(ConverterErr);
        }
        Ok(Self { range_and_offset })
    }
}
#[derive(Debug, Default)]
struct Almanac {
    converters_list: Vec<Vec<Converter>>,
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let mut out = Self::default();

        // let headerIterBlock = input.lines().take(2);
        let mut lines = input.lines();
        let h1 = lines.next();
        let h2 = lines.next();

        // assert than a title line is observed ( a line with a ':')

        // Collect maps
        loop {
            let have_title = lines.next().unwrap().contains(':');
            if !have_title {
                panic!("was expecting a title");
            }

            loop {
                let mut converters: Vec<Converter> = vec![];
                match lines.next() {
                    Some(line) => {
                        dbg!(&line);
                        // help
                        match line.try_into() {
                            Ok(converter) => {
                                // hhh
                                converters.push(converter);
                            }
                            Err(_) => {
                                // blank line
                                dbg!("Failed to convert");
                                break;
                                // panic!("Could not convert line into a converter");
                            }
                        };
                    }
                    None => {
                        // eof
                        break;
                    }
                }
            }
        }
        out
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

        let definition = r"50 98 2
52 50 48";

        if let Ok(converter) = Converter::try_from(definition) {
            for [input, expected] in dataset {
                let actual = converter.convert(input);
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    fn test_blank_line_fails_to_convert() {
        let input = "";
        if let Ok(c) = Converter::try_from(input) {
            dbg!(c);
            assert!(false);
        } else {
            assert!(true);
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

        for [seed, location] in seed_location_reference {
            let computed_location = almanac.location_from_seed(&seed);
            assert_eq!(location, computed_location);
        }
    }
}
