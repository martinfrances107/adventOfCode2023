use core::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("./input1.txt");
    println!("{:?}", part1(input));
}

fn part1(input: &str) -> u64 {
    let mut hand_bid = input
        .lines()
        .map(|line| {
            let (hand_str, bid_str) = line.split_once(' ').expect("one wite space");
            let hand: Hand = hand_str.into();
            let bid = bid_str.parse::<u64>().expect("failed to parse bid");
            (hand, bid)
        })
        .collect::<Vec<(Hand, u64)>>();

    hand_bid.sort_by(|(hand_a, _bid_a), (hand_b, _bid_b)| hand_a.cmp(hand_b));

    // assert_eq!(hands.len(), 1000);
    let data = hand_bid
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| {
            let rank = (i as u64) + 1;

            rank * bid
        })
        .collect::<Vec<_>>();

    data.iter().sum()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    Ace,
}

impl From<&char> for Card {
    fn from(x: &char) -> Self {
        match x {
            'A' => Card::Ace,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => {
                panic!("malformed card")
            }
        }
    }
}
#[derive(Debug, Eq)]
enum Hand {
    HighCard([Card; 5]),
    OnePair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    FourOfAKind([Card; 5]),
    FiveOfAKind([Card; 5]),
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(&other) {
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Hand::HighCard(c0) => match other {
                Hand::HighCard(c1) => Some(c0.cmp(c1)),
                Hand::OnePair(_)
                | Hand::TwoPair(_)
                | Hand::ThreeOfAKind(_)
                | Hand::FullHouse(_)
                | Hand::FourOfAKind(_)
                | Hand::FiveOfAKind(_) => Some(Ordering::Less),
            },
            Hand::OnePair(c0) => match other {
                Hand::HighCard(_) => Some(Ordering::Greater),
                Hand::OnePair(c1) => Some(c0.cmp(c1)),
                Hand::TwoPair(_)
                | Hand::ThreeOfAKind(_)
                | Hand::FullHouse(_)
                | Hand::FourOfAKind(_)
                | Hand::FiveOfAKind(_) => Some(Ordering::Less),
            },
            Hand::TwoPair(c0) => match other {
                Hand::HighCard(_) | Hand::OnePair(_) => Some(Ordering::Greater),
                Hand::TwoPair(c1) => Some(c0.cmp(c1)),
                Hand::ThreeOfAKind(_)
                | Hand::FullHouse(_)
                | Hand::FourOfAKind(_)
                | Hand::FiveOfAKind(_) => Some(Ordering::Less),
            },
            Hand::ThreeOfAKind(c0) => match other {
                Hand::HighCard(_) | Hand::OnePair(_) | Hand::TwoPair(_) => Some(Ordering::Greater),
                Hand::ThreeOfAKind(c1) => Some(c0.cmp(c1)),
                Hand::FullHouse(_) | Hand::FourOfAKind(_) | Hand::FiveOfAKind(_) => {
                    Some(Ordering::Less)
                }
            },
            Hand::FullHouse(c0) => match other {
                Hand::HighCard(_) | Hand::OnePair(_) | Hand::TwoPair(_) | Hand::ThreeOfAKind(_) => {
                    Some(Ordering::Greater)
                }
                Hand::FullHouse(c1) => Some(c0.cmp(c1)),
                Hand::FourOfAKind(_) | Hand::FiveOfAKind(_) => Some(Ordering::Less),
            },
            Hand::FourOfAKind(c0) => match other {
                Hand::HighCard(_)
                | Hand::OnePair(_)
                | Hand::TwoPair(_)
                | Hand::ThreeOfAKind(_)
                | Hand::FullHouse(_) => Some(Ordering::Greater),
                Hand::FourOfAKind(c1) => Some(c0.cmp(c1)),
                Hand::FiveOfAKind(_) => Some(Ordering::Less),
            },
            Hand::FiveOfAKind(c0) => match other {
                Hand::HighCard(_)
                | Hand::OnePair(_)
                | Hand::TwoPair(_)
                | Hand::ThreeOfAKind(_)
                | Hand::FullHouse(_)
                | Hand::FourOfAKind(_) => Some(Ordering::Greater),
                Hand::FiveOfAKind(c1) => Some(c0.cmp(c1)),
            },
        }
    }
}
impl From<&str> for Hand {
    fn from(line: &str) -> Hand {
        let mut hand = [Card::Two; 5];
        for (i, c) in line.chars().take(5).enumerate() {
            hand[i] = (&c).into();
        }

        let unique_cards: BTreeSet<Card> = hand.map(|c| c).into();

        let mut histogram = BTreeMap::new();
        for uc in unique_cards {
            for h in hand.iter() {
                if uc == *h {
                    // increment tally.
                    match histogram.get_mut(&uc) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            histogram.insert(uc, 1);
                        }
                    }
                }
            }
        }

        let heap: BinaryHeap<_> = histogram
            .iter()
            .map(|(card, count)| {
                // Reverse order so when sorted
                // it produces a list sorted by group.
                (count, card)
            })
            .collect();

        let mut list = heap.into_sorted_vec();
        list.reverse();

        // Extract from the histogram the most and second most common card.

        let mut iter = list.iter();
        let (r1, _c1) = iter
            .next()
            .expect("There must always be a primary grouping");
        let r2_c2 = iter.next();

        match (r1, r2_c2) {
            (5, None) => Hand::FiveOfAKind(hand),
            (4, Some((1, _c2))) => Hand::FourOfAKind(hand),
            (3, Some((&2, _c2))) => Hand::FullHouse(hand),
            (3, Some((1, _c2))) => Hand::ThreeOfAKind(hand),
            (2, Some((2, _c2))) => Hand::TwoPair(hand),
            (2, Some((1, _spare0))) => Hand::OnePair(hand),
            (1, Some((1, _spare0))) => Hand::HighCard(hand),
            _ => {
                panic!("bad decode");
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_calc_hands() {
        const TESTSET: [(Hand, &str); 3] = [
            (
                Hand::TwoPair([Card::Two, Card::Three, Card::Four, Card::Three, Card::Two]),
                "23432",
            ),
            (
                Hand::OnePair([Card::Ace, Card::Two, Card::Three, Card::Ace, Card::Four]),
                "A23A4",
            ),
            (
                Hand::FourOfAKind([Card::Ace, Card::Ace, Card::Eight, Card::Ace, Card::Ace]),
                "AA8AA",
            ),
            // (Hand::FullHouse(Card::Two), "23332"),
            // (Hand::ThreeOfAKind(Card::T), "TTT98"),
            // (Hand::HighCard(Card::Two), "23456"),
            // (Hand::FiveOfAKind(Card::Ace), "AAAAA"),
        ];

        for (h, line) in TESTSET {
            assert_eq!(h, line.into())
        }
    }

    #[test]
    fn calc_rank() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 48";

        let mut hands = input
            .lines()
            .map(|line| {
                let (hand_str, bid_str) = line.split_once(' ').expect("one white space");
                let hand: Hand = hand_str.into();
                let _bid = bid_str.parse::<u32>().expect("failed to parse bid");
                hand
            })
            .collect::<Vec<Hand>>();
        // Ranked hands are sorted hands.

        hands.sort();

        let expected_ranked_hanks = vec![
            Hand::OnePair([Card::Three, Card::Two, Card::T, Card::Three, Card::K]),
            Hand::TwoPair([Card::K, Card::T, Card::J, Card::J, Card::T]),
            Hand::TwoPair([Card::K, Card::K, Card::Six, Card::Seven, Card::Seven]),
            Hand::ThreeOfAKind([Card::T, Card::Five, Card::Five, Card::J, Card::Five]),
            Hand::ThreeOfAKind([Card::Q, Card::Q, Card::Q, Card::J, Card::Ace]),
        ];

        assert_eq!(expected_ranked_hanks, hands);
    }

    #[test]
    fn calc_sum() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let total = part1(input);
        assert_eq!(total, 6440);
    }

    #[test]
    fn cherry_picking() {
        // edges cases seen in the input.txt file

        assert_eq!(
            Hand::FiveOfAKind([Card::J, Card::J, Card::J, Card::J, Card::J]),
            r"JJJJJ".into()
        );
        assert_eq!(
            Hand::TwoPair([Card::T, Card::Ace, Card::T, Card::Ace, Card::Six]),
            r"TATA6".into(),
        );
    }

    #[test]
    fn missing() {
        // Missing examples not seen in test
        // five of a kind
        // assert_eq!(Hand::FiveOfAKind(Card::J), r"JJJJJ".into());
        // // 4 of a kind
        // assert_eq!(Hand::FourOfAKind(Card::Two), r"22A22".into(),);
        // // Full house
        // assert_eq!(Hand::FullHouse(Card::T), r"TTAAA".into(),);
        // // High card
        // assert_eq!(Hand::HighCard(Card::Two), r"23456".into(),);
    }
    #[test]
    fn comparison() {
        // So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger
        let high: Hand = r"33332".into();
        let low: Hand = r"2AAAA".into();
        assert_eq!(high.cmp(&low), Ordering::Greater);
        assert_eq!(high.cmp(&high), Ordering::Equal);
        assert_eq!(low.cmp(&high), Ordering::Less);

        let high: Hand = r"6543A".into();
        let low: Hand = r"65432".into();
        assert_eq!(high.cmp(&low), Ordering::Greater);
        assert_eq!(high.cmp(&high), Ordering::Equal);
        assert_eq!(low.cmp(&high), Ordering::Less);
    }

    #[test]
    fn w_ordering() {
        let high = Hand::ThreeOfAKind([Card::T, Card::Five, Card::Five, Card::J, Card::Five]);
        let low = Hand::TwoPair([Card::K, Card::K, Card::Six, Card::Seven, Card::Seven]);
        assert_eq!(high.cmp(&low), Ordering::Greater);
        assert_eq!(high.cmp(&high), Ordering::Equal);
        assert_eq!(low.cmp(&low), Ordering::Equal);
        assert_eq!(low.cmp(&high), Ordering::Less);

        let high = Hand::FiveOfAKind([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]);
        let low = Hand::HighCard([Card::Ace, Card::T, Card::Nine, Card::Eight, Card::Seven]);
        assert_eq!(high.cmp(&low), Ordering::Greater);
        assert_eq!(high.cmp(&high), Ordering::Equal);
        assert_eq!(low.cmp(&low), Ordering::Equal);
        assert_eq!(low.cmp(&high), Ordering::Less);
    }
}
