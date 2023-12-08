use crate::Solution;
use anyhow::Result;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;

pub fn solve(part: u8, input: &'static str) -> Result<Solution, anyhow::Error> {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/07.input")
    };

    if part == 1 {
        // {{{ Card

        // A:14, K:13, Q:12, J:11, T:10
        // 9, 8, 7, 6, 5, 4, 3, or 2
        #[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
        struct Card(u8);
        impl Card {
            pub fn from(c: char) -> Self {
                match c.to_digit(10) {
                    Some(v) => Card(v as u8),
                    None => match c {
                        'A' => Card(14),
                        'K' => Card(13),
                        'Q' => Card(12),
                        'J' => Card(11),
                        'T' => Card(10),
                        _ => panic!("invalid letter"),
                    },
                }
            }
        }

        // }}}
        // {{{ Hand

        #[derive(Debug, Clone, Copy)]
        #[repr(u8)]
        enum Kind {
            HighCard,
            OnePair(Card),
            TwoPair(Card, Card),
            Three(Card),
            FullHouse(Card, Card),
            Four(Card),
            Five(Card),
        }
        impl Kind {
            fn discriminant(&self) -> u8 {
                unsafe { *<*const _>::from(self).cast::<u8>() }
            }
        }
        #[derive(Debug, Clone, Copy)]
        struct Hand {
            cards: [Card; 5],
            kind: Kind,
            bid: u16,
        }
        impl Hand {
            pub fn new(input: &str) -> Self {
                let mut i = input.chars();
                let cards = [
                    Card::from(i.next().expect("no char 1")),
                    Card::from(i.next().expect("no char 2")),
                    Card::from(i.next().expect("no char 3")),
                    Card::from(i.next().expect("no char 4")),
                    Card::from(i.next().expect("no char 5")),
                ];
                assert_eq!(i.next(), Some(' '));
                let bid = i.as_str().parse::<u16>().expect("bid");

                let mut map = FxHashMap::default();
                for card in &cards {
                    map.entry(*card).and_modify(|x| *x += 1).or_insert(1u8);
                }
                let cmp = |a: &(&Card, &u8), b: &(&Card, &u8)| a.1.cmp(&b.1);
                let max = map.iter().max_by(cmp).unwrap();
                let kind = match max {
                    (card, 5) => Kind::Five(*card),
                    (card, 4) => Kind::Four(*card),
                    (card, 3) => {
                        match map.iter().filter(|(k, _)| *k != card).max_by(cmp).unwrap() {
                            (card2, 2) => Kind::FullHouse(*card, *card2),
                            (card2, 1) => Kind::Three(*card),
                            _ => unreachable!(),
                        }
                    }
                    (card, 2) => {
                        match map.iter().filter(|(k, _)| *k != card).max_by(cmp).unwrap() {
                            (card2, 2) => Kind::TwoPair(*card, *card2),
                            (card2, 1) => Kind::OnePair(*card),
                            _ => unreachable!(),
                        }
                    }
                    (card, 1) => Kind::HighCard,
                    _ => unreachable!(),
                };

                Self { cards, kind, bid }
            }
        }
        impl Eq for Hand {}
        impl PartialEq for Hand {
            fn eq(&self, other: &Self) -> bool {
                self.cards == other.cards
            }
        }
        impl PartialOrd for Hand {
            fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
                Some(
                    self.kind
                        .discriminant()
                        .cmp(&other.kind.discriminant())
                        .then_with(|| {
                            for i in 0..5 {
                                let cmp = self.cards[i].0.cmp(&other.cards[i].0);
                                if cmp != Ordering::Equal {
                                    return cmp;
                                }
                            }
                            Ordering::Equal
                        }),
                )
            }
        }
        impl Ord for Hand {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(&other).unwrap()
            }
        }

        // }}}

        let mut hands = input.lines().map(Hand::new).collect::<Vec<_>>();
        hands.sort();
        let res = hands
            .iter()
            .enumerate()
            .map(|(idx, x)| (idx + 1) * (x.bid as usize))
            .sum::<usize>();

        Ok(Solution::U64(res as u64))
    } else {
        // {{{ Card

        // A:14, K:13, Q:12, J:11→1, T:10
        // 9, 8, 7, 6, 5, 4, 3, or 2
        #[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
        struct Card(u8);
        impl Card {
            pub fn from(c: char) -> Self {
                match c.to_digit(10) {
                    Some(v) => Card(v as u8),
                    None => match c {
                        'A' => Card(14),
                        'K' => Card(13),
                        'Q' => Card(12),
                        'J' => Card(1),
                        'T' => Card(10),
                        _ => panic!("invalid letter"),
                    },
                }
            }
        }

        // }}}
        // {{{ Hand

        #[derive(Debug, Clone, Copy)]
        #[repr(u8)]
        enum Kind {
            HighCard,
            OnePair(Card),
            TwoPair(Card, Card),
            Three(Card),
            FullHouse(Card, Card),
            Four(Card),
            Five(Card),
        }
        impl Kind {
            fn discriminant(&self) -> u8 {
                unsafe { *<*const _>::from(self).cast::<u8>() }
            }
        }
        #[derive(Debug, Clone, Copy)]
        struct Hand {
            cards: [Card; 5],
            kind: Kind,
            bid: u16,
        }
        impl Hand {
            pub fn new(input: &str) -> Self {
                let mut i = input.chars();
                let cards = [
                    Card::from(i.next().expect("no char 1")),
                    Card::from(i.next().expect("no char 2")),
                    Card::from(i.next().expect("no char 3")),
                    Card::from(i.next().expect("no char 4")),
                    Card::from(i.next().expect("no char 5")),
                ];
                assert_eq!(i.next(), Some(' '));
                let bid = i.as_str().parse::<u16>().expect("bid");

                let mut map = FxHashMap::default();
                for card in &cards {
                    map.entry(*card).and_modify(|x| *x += 1).or_insert(1u8);
                }
                let cmp = |a: &(&Card, &u8), b: &(&Card, &u8)| a.1.cmp(&b.1);
                let joker = Card::from('J');
                let max = map
                    .iter()
                    .filter(|(k, _)| **k != joker)
                    .max_by(cmp)
                    .unwrap_or((&joker, &0));
                let boost_joker = *map.get(&joker).unwrap_or(&0);
                let kind = match (max.0, max.1 + boost_joker) {
                    (card, 5..) => Kind::Five(*card),
                    (card, 4) => Kind::Four(*card),
                    (card, 3) => {
                        match map
                            .iter()
                            .filter(|(k, _)| *k != card && **k != joker)
                            .max_by(cmp)
                            .unwrap()
                        {
                            (card2, 2) => Kind::FullHouse(*card, *card2),
                            (card2, 1) => Kind::Three(*card),
                            _ => unreachable!(),
                        }
                    }
                    (card, 2) => {
                        match map
                            .iter()
                            .filter(|(k, _)| *k != card && **k != joker)
                            .max_by(cmp)
                            .unwrap()
                        {
                            (card2, 2) => Kind::TwoPair(*card, *card2),
                            (card2, 1) => Kind::OnePair(*card),
                            _ => unreachable!(),
                        }
                    }
                    (card, 1) => Kind::HighCard,
                    _ => unreachable!(),
                };

                Self { cards, kind, bid }
            }
        }
        impl Eq for Hand {}
        impl PartialEq for Hand {
            fn eq(&self, other: &Self) -> bool {
                self.cards == other.cards
            }
        }
        impl PartialOrd for Hand {
            fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
                Some(
                    self.kind
                        .discriminant()
                        .cmp(&other.kind.discriminant())
                        .then_with(|| {
                            for i in 0..5 {
                                let cmp = self.cards[i].0.cmp(&other.cards[i].0);
                                if cmp != Ordering::Equal {
                                    return cmp;
                                }
                            }
                            Ordering::Equal
                        }),
                )
            }
        }
        impl Ord for Hand {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(&other).unwrap()
            }
        }

        // }}}

        let mut hands = input.lines().map(Hand::new).collect::<Vec<_>>();
        hands.sort();
        let res = hands
            .iter()
            .enumerate()
            .map(|(idx, x)| (idx + 1) * (x.bid as usize))
            .sum::<usize>();

        Ok(Solution::U64(res as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn part_1() {
        assert_eq!(solve(1, SAMPLE).unwrap(), Solution::U64(6440));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, SAMPLE).unwrap(), Solution::U64(5905));
    }
}
