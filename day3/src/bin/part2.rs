use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

type GearKey = (usize, usize);

#[derive(Debug, Default)]
struct Board<'a> {
    total: u32,
    last: BoardRow<'a>,
    active: BoardRow<'a>,
    next: BoardRow<'a>,
    gears: HashMap<GearKey, Vec<u32>>,
}

#[derive(Clone, Debug, Default)]
struct BoardRow<'a> {
    line_num: usize,
    line: &'a str,
}

impl<'a> BoardRow<'a> {
    // Search board row and report if a gear is found.
    fn find_gear(&self, start: usize, end: usize) -> Option<usize> {
        if let Some(segment) = self.line.get(start..end) {
            for (pos, char) in segment.chars().enumerate() {
                if char == '*' {
                    return Some(pos);
                }
            }
        }
        None
    }
}

impl<'a> Board<'a> {
    fn update(&mut self, line_num: usize, line: &'a str) {
        let mut new = Board {
            last: self.active.clone(),
            active: self.next.clone(),
            next: BoardRow { line_num, line },

            total: self.total,
            gears: self.gears.clone(),
        };
        new.process_active_line();

        *self = new;
    }

    fn process_active_line(&mut self) {
        let mut logging_number = false;
        let mut number_start = 0;
        for (i, char) in self.active.line.chars().enumerate() {
            if char.is_ascii_digit() {
                if !logging_number {
                    logging_number = true;
                    number_start = i;
                }
            } else {
                if logging_number {
                    // Number capture is ending
                    let number = self.active.line[number_start..i]
                        .parse::<u32>()
                        .expect("must have a valid number at this point");
                    //Now we have a search range for a row
                    // look above and below and see if we have a part number.
                    let search_start = if number_start == 0 {
                        0usize
                    } else {
                        number_start - 1
                    };
                    let search_end = if i == self.active.line.len() {
                        i
                    } else {
                        i + 1
                    };

                    if let Some(offset) = self.last.find_gear(search_start, search_end) {
                        let gear_key = (self.last.line_num, search_start + offset);
                        // println!("[last] found gear at {gear_key:#?}");
                        // Found a number attached to a gear.
                        match self.gears.get_mut(&gear_key) {
                            Some(gear) => {
                                // print!("pushing to existing gear");
                                gear.push(number);
                            }
                            None => {
                                // print!("creating gear");
                                // dbg!(gear_key);
                                // dbg!(vec![number]);
                                self.gears.insert(gear_key, vec![number]);
                            }
                        }
                    }
                    if let Some(offset) = self.active.find_gear(search_start, search_end) {
                        let gear_key = (self.active.line_num, search_start + offset);
                        // println!("[last] found gear at {gear_key:#?}");
                        // Found a number attached to a gear.
                        match self.gears.get_mut(&gear_key) {
                            Some(gear) => {
                                // print!("pushing to existing gear");
                                gear.push(number);
                            }
                            None => {
                                // print!("creating gear");
                                // dbg!(gear_key);
                                // dbg!(vec![number]);
                                self.gears.insert(gear_key, vec![number]);
                            }
                        }
                    }

                    if let Some(offset) = self.next.find_gear(search_start, search_end) {
                        let gear_key = (self.next.line_num, search_start + offset);
                        // println!("[last] found gear at {gear_key:#?}");
                        // Found a number attached to a gear.
                        match self.gears.get_mut(&gear_key) {
                            Some(gear) => {
                                // print!("pushing to existing gear");
                                gear.push(number);
                            }
                            None => {
                                // print!("creating gear");
                                // dbg!(gear_key);
                                // dbg!(vec![number]);
                                self.gears.insert(gear_key, vec![number]);
                            }
                        }
                    }
                }
                logging_number = false;
            }
        }
        if logging_number {
            let i = self.last.line.len();
            // Number capture is ending
            let number = self.active.line[number_start..i]
                .parse::<u32>()
                .expect("must have a valid number at this point");
            //Now we have a search range for a row
            // look above and below and see if we have a part number.
            let search_start = if number_start == 0 {
                0usize
            } else {
                number_start - 1
            };
            let search_end = if i == self.active.line.len() {
                i
            } else {
                i + 1
            };

            if let Some(offset) = self.last.find_gear(search_start, search_end) {
                let gear_key = (self.last.line_num, search_start + offset);
                // println!("[last] found gear at {gear_key:#?}");
                // Found a number attached to a gear.
                match self.gears.get_mut(&gear_key) {
                    Some(gear) => {
                        // print!("pushing to existing gear");
                        gear.push(number);
                    }
                    None => {
                        // print!("creating gear");
                        // dbg!(gear_key);
                        // dbg!(vec![number]);
                        self.gears.insert(gear_key, vec![number]);
                    }
                }
            }
            if let Some(offset) = self.active.find_gear(search_start, search_end) {
                let gear_key = (self.active.line_num, search_start + offset);
                // println!("[last] found gear at {gear_key:#?}");
                // Found a number attached to a gear.
                match self.gears.get_mut(&gear_key) {
                    Some(gear) => {
                        // print!("pushing to existing gear");
                        gear.push(number);
                    }
                    None => {
                        // print!("creating gear");
                        // dbg!(gear_key);
                        // dbg!(vec![number]);
                        self.gears.insert(gear_key, vec![number]);
                    }
                }
            }

            if let Some(offset) = self.next.find_gear(search_start, search_end) {
                let gear_key = (self.next.line_num, search_start + offset);
                // println!("[last] found gear at {gear_key:#?}");
                // Found a number attached to a gear.
                match self.gears.get_mut(&gear_key) {
                    Some(gear) => {
                        // print!("pushing to existing gear");
                        gear.push(number);
                    }
                    None => {
                        // print!("creating gear");
                        // dbg!(gear_key);
                        // dbg!(vec![number]);
                        self.gears.insert(gear_key, vec![number]);
                    }
                }
            }

            logging_number = false;
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut board = Board::default();
    for (line_num, line) in input.lines().enumerate() {
        board.update(line_num, line);
    }
    // push a blank line into update so the last last become the active line.
    let line_len = board.active.line.len();
    dbg!(line_len);

    // created a blank line with 140 dots
    let blank_line = ['.'; 140].into_iter().map(|c| c).collect::<String>();
    dbg!(&blank_line);
    board.update(board.next.line_num + 1, &blank_line);

    dbg!("have gear list");
    // Valid gear has 2 associated numbers.
    let total: u32 = board
        .gears
        .drain()
        .filter(|(_key, numbers)| numbers.len() == 2)
        .map(|(_key, numbers)| numbers[0] * numbers[1])
        .sum();
    dbg!(total);
    total
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
        assert_eq!(part1(input), 467835u32)
    }
}
