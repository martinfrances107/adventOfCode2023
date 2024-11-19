use std::collections::HashMap;

use nom::branch::alt;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res},
    error::Error,
    sequence::tuple,
    Err, IResult,
};

use nom::error::ErrorKind;

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    LT,
    EQ,
    GT,
}

#[derive(Clone, Debug, PartialEq)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Debug, PartialEq)]
struct Rule<'a> {
    category: Category,
    operator: Operator,
    value: u32,
    next: &'a str,
}

impl<'a> Rule<'a> {
    fn rule_applies(&self, part: &Part) -> bool {
        match self.operator {
            Operator::EQ => match self.category {
                Category::X => part.x == self.value,
                Category::M => part.m == self.value,
                Category::A => part.a == self.value,
                Category::S => part.s == self.value,
            },
            Operator::GT => match self.category {
                Category::X => part.x > self.value,
                Category::M => part.m > self.value,
                Category::A => part.a > self.value,
                Category::S => part.s > self.value,
            },
            Operator::LT => match self.category {
                Category::X => part.x < self.value,
                Category::M => part.m < self.value,
                Category::A => part.a < self.value,
                Category::S => part.s < self.value,
            },
        }
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
impl Part {
    fn value(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}
fn parse_part(input: &str) -> IResult<&str, Part> {
    map(
        tuple((
            tag("{x="),
            parse_value,
            tag(",m="),
            parse_value,
            tag(",a="),
            parse_value,
            tag(",s="),
            parse_value,
            tag("}"),
        )),
        |(_, x, _, m, _, a, _, s, _)| {
            // to
            Part { x, m, a, s }
        },
    )(input)
}
fn main() {
    let input = include_str!("./input1.txt");

    println!("{:?}", part1(input));
}

// Needs span/desroy algorithm
fn parse_workflow_name(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_categories(input: &str) -> IResult<&str, Category> {
    map_res(alt((char('x'), char('m'), char('a'), char('s'))), |c| {
        Ok(match c {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => return Err(Err::Error(Error::new(input, ErrorKind::Alt))),
        })
    })(input)
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    map_res(alt((char('>'), char('='), char('<'))), |c| {
        Ok(match c {
            '<' => Operator::LT,
            '=' => Operator::EQ,
            '>' => Operator::GT,
            _ => return Err(Err::Error(Error::new(input, ErrorKind::Alt))),
        })
    })(input)
}

fn parse_value(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(
        tuple((
            parse_categories,
            parse_operator,
            parse_value,
            tag(":"),
            parse_workflow_name,
        )),
        |(category, operator, value, _colon, name)| Rule {
            category,
            operator,
            value,
            next: name,
        },
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<Rule>, &str)> {
    map(
        tuple((
            parse_workflow_name,
            tag("{"),
            separated_list1(tag(","), parse_rule),
            tag(","),
            parse_workflow_name,
            tag("}"),
        )),
        |(name, _open, rules, _comma, failed_rule, _close)| (name, rules, failed_rule),
    )(input)
}

fn part1(input: &str) -> u32 {
    let rules = include_str!("./rules1.txt");
    let named_rules: Vec<(&str, Vec<Rule>, &str)> = rules
        .lines()
        .map(|line| {
            println!("line  - {line:#?}");
            parse_line(line)
        })
        .map(|result| {
            if let Ok((_remain, (name, rules, failed))) = result {
                (name, rules, failed)
            } else {
                panic!("part1 Failed to parse line");
            }
        })
        .collect::<Vec<_>>();

    let mut rule_set = HashMap::new();
    for (name, rules, failed_rule) in named_rules {
        rule_set.insert(name, (rules, failed_rule));
    }

    let parts_str = include_str!("./parts1.txt");

    let mut total = 0;
    for part_str in parts_str.lines() {
        let (_remain, part) = parse_part(part_str).expect("must have valid part.");
        println!("Part {:#?}", part);

        if accept_part(&rule_set, &part) {
            total += part.value();
        }
    }
    println!("total {total:?}");
    todo!();
}

fn accept_part(rule_set: &HashMap<&str, (Vec<Rule>, &str)>, part: &Part) -> bool {
    let mut rule_name = "in";

    let mut loop_count = 0;

    'rule_search: loop {
        println!("testing rule {rule_name:?}");
        if rule_name == "A" || rule_name == "R" {
            return rule_name == "A";
        }
        let (r, fr) = rule_set.get(rule_name).expect("Must have a in rule");
        let rules = r.clone();
        let failed_rule = *fr;

        for rule in &rules {
            if rule.rule_applies(part) {
                rule_name = rule.next;
                // Return to top and follow next rule
                continue 'rule_search;
            }
        }

        // All the rules followed but no match - from here failing rule applies
        if failed_rule == "A" || failed_rule == "R" {
            // break 'rule_search;
            return failed_rule == "A";
        }
        rule_name = failed_rule;

        if loop_count > 10u16 {
            panic!("Failed loop count {:?}", loop_count);
        }
        loop_count += 1;
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn rule() {
        let parsed = parse_rule("a<2006:qkq");
        if let Ok((_remain, rule)) = parsed {
            assert_eq!(
                rule,
                Rule {
                    category: Category::A,
                    operator: Operator::LT,
                    value: 2006,
                    next: "qkq"
                }
            );
        }
    }

    #[test]
    fn with_failed_rule() {
        let parsed = parse_line("px{a<2006:qkq,m>2090:A,rfg}");
        if let Ok((_remain2, (name, rules, failed_rule))) = parsed {
            assert_eq!(name, "px");

            assert_eq!(
                rules,
                vec![
                    Rule {
                        category: Category::A,
                        operator: Operator::LT,
                        value: 2006,
                        next: "qkq"
                    },
                    Rule {
                        category: Category::M,
                        operator: Operator::GT,
                        value: 2090,
                        next: "A"
                    },
                ]
            );
            assert_eq!(failed_rule, "rfg");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn example() {
        let rules = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}";
        let named_rules: Vec<(&str, Vec<Rule>, &str)> = rules
            .lines()
            .map(|line| {
                println!("line  - {line:#?}");
                parse_line(line)
            })
            .map(|result| {
                if let Ok((_remain, (name, rules, failed))) = result {
                    (name, rules, failed)
                } else {
                    panic!("part1 Failed to parse line");
                }
            })
            .collect::<Vec<_>>();

        let mut rule_set = HashMap::new();
        for (name, rules, failed_rule) in named_rules {
            rule_set.insert(name, (rules, failed_rule));
        }

        let part_pass = [
            ("{x=787,m=2655,a=1222,s=2876}", true),
            ("{x=1679,m=44,a=2067,s=496}", false),
            ("{x=2036,m=264,a=79,s=2244}", true),
            ("{x=2461,m=1339,a=466,s=291}", false),
            ("{x=2127,m=1623,a=2188,s=1013}", true),
        ];

        let mut total = 0;
        for (part_str, expected) in part_pass {
            let (_remain, part) = parse_part(part_str).expect("must have valid part.");
            println!("Part {:#?}", part);

            let is_accepted = accept_part(&rule_set, &part);
            assert_eq!(accept_part(&rule_set, &part), expected);

            if is_accepted {
                total += part.value();
            }
        }
        println!("total {total:?}");
        assert_eq!(total, 19114);
    }
}
