use core::{ops::Range, slice::SliceIndex};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

#[derive(Debug, Default)]
struct Board<'a> {
    total: u32,
    last: BoardRow<'a>,
    active: BoardRow<'a>,
    next: BoardRow<'a>,
}

#[derive(Clone, Debug, Default)]
struct BoardRow<'a>(&'a str);

impl<'a> BoardRow<'a> {
    fn has_part_number(&self, start: usize, end: usize) -> bool {
        match self.0.get(start..end) {
            Some(segment) => {
                for c in segment.chars() {
                    if !c.is_digit(10) && c != '.' {
                        println!("part symbol {c}");
                        return true;
                    }
                }
            }
            None => return false,
        }
        false
    }
}

impl<'a> Board<'a> {
    fn update(&mut self, line: &'a str) {
        let mut new = Board {
            total: self.total,
            last: self.active.clone(),
            active: self.next.clone(),
            next: BoardRow(line),
        };
        new.process_active_line();

        *self = new;
    }

    fn process_active_line(&mut self) {
        let mut logging_number = false;
        let mut number_start = 0;
        for (i, char) in self.active.0.chars().enumerate() {
            if char.is_digit(10) {
                if logging_number == false {
                    logging_number = true;
                    number_start = i;
                }
            } else {
                if logging_number {
                    // Number capture is ending
                    let number = self.active.0[number_start..i]
                        .parse::<u32>()
                        .expect("must have a valid number at this point");
                    //Now we have a search range for a row
                    // look above and below and see if we have a part number.
                    let search_start = if number_start == 0 {
                        0usize
                    } else {
                        number_start - 1
                    };
                    let search_end = if i == self.active.0.len() { i } else { i + 1 };
                    let is_part_number = self.last.has_part_number(search_start, search_end)
                        || self.active.has_part_number(search_start, search_end)
                        || self.next.has_part_number(search_start, search_end);

                    if is_part_number {
                        self.total += number;
                        println!("extracted number {number:#?}");
                    }
                }
                logging_number = false;
            }
        }
        if logging_number {
            let i = self.active.0.len();
            let number = self.active.0[number_start..i]
                .parse::<u32>()
                .expect("must have a valid number at this point");
            //Now we have a search range for a row
            // look above and below and see if we have a part number.
            let search_start = if number_start == 0 {
                0usize
            } else {
                number_start - 1
            };
            let search_end = if i == self.active.0.len() { i } else { i + 1 };
            let is_part_number = self.last.has_part_number(search_start, search_end)
                || self.active.has_part_number(search_start, search_end)
                || self.next.has_part_number(search_start, search_end);

            if is_part_number {
                self.total += number;
                println!("extracted number {number:#?}");
            }
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut board = Board::default();
    for line in input.lines() {
        board.update(line);
        // println!("{board:#?}");
    }
    // push a blank line into update so the last last become the active line.
    board.update("..........");
    // println!("final total {}", board.total);
    board.total
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn example() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(input), 4361u32)
    }

    #[test]
    // 633 is shift on to the right
    fn number_on_right_boundary() {
        let input = r"467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(input), 4361u32)
    }
}
