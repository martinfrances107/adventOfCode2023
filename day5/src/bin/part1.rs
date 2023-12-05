use core::ops::Range;
fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

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

impl From<&str> for Converter {
    fn from(input: &str) -> Self {
        let mut range_and_offset = vec![];
        for line in input.lines() {
            let mut inputs = line.split(' ').filter_map(|x| x.parse::<i128>().ok());
            let dst = inputs.next().expect("dst must be present");
            let start = inputs.next().expect("start must be present");
            let range_len = inputs.next().expect("range must be present");
            let end = start + range_len;
            let offset = dst - start;
            range_and_offset.push((start..end, offset));
        }
        Self { range_and_offset }
    }
}
struct Almanac {
    converters: Vec<Converter>,
}

impl From<&str> for Almanac {
    fn from(_input: &str) -> Self {
        todo!();
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

        let converter: Converter = definition.into();
        for [input, expected] in dataset {
            let actual = converter.convert(input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    #[ignore]
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
        assert_eq!(part1(input), 142u32)
    }
}
