use std::cmp::Ordering;
use std::iter::zip;

fn main() {
    let input = include_str!("../../input/day7.txt");

    assert_eq!(part1(input), 256448566);
    assert_eq!(part2(input), 254412181);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

use HandKind::*;

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    kind: HandKind,
    bid: u32,
}

fn solve(mut hands: Vec<Hand>) -> u32 {
    hands.sort_by(|a, b| match a.kind.cmp(&b.kind) {
        Ordering::Equal => a.cards.cmp(&b.cards),
        ord => ord,
    });

    hands
        .iter()
        .zip(1..)
        .map(|(hand, rank)| rank * hand.bid)
        .sum()
}

fn part1(input: &str) -> u32 {
    let hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let (symbols, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            let mut cards = [0; 5];
            let mut counter = [0; 13];
            for (sym, card) in zip(symbols.as_bytes(), &mut cards) {
                *card = match sym {
                    b'2'..=b'9' => sym - 50,
                    b'T' => 8,
                    b'J' => 9,
                    b'Q' => 10,
                    b'K' => 11,
                    b'A' => 12,
                    _ => unreachable!(),
                };
                counter[*card as usize] += 1;
            }

            let mut kind = HighCard;
            let mut three = false;
            let mut two = 0;
            for count in counter {
                match count {
                    5 => kind = FiveOfAKind,
                    4 => kind = FourOfAKind,
                    3 => three = true,
                    2 => two += 1,
                    0 | 1 => {}
                    _ => unreachable!(),
                }
            }

            if kind == HighCard {
                kind = match (three, two) {
                    (false, 0) => HighCard,
                    (false, 1) => OnePair,
                    (false, 2) => TwoPair,
                    (true, 0) => ThreeOfAKind,
                    (true, 1) => FullHouse,
                    _ => unreachable!(),
                };
            }

            Hand { cards, kind, bid }
        })
        .collect();

    solve(hands)
}

fn part2(input: &str) -> u32 {
    let hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let (symbols, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            let mut cards = [0; 5];
            let mut counter = [0; 13];
            for (sym, card) in zip(symbols.as_bytes(), &mut cards) {
                *card = match sym {
                    b'J' => 0,
                    b'2'..=b'9' => sym - 49,
                    b'T' => 9,
                    b'Q' => 10,
                    b'K' => 11,
                    b'A' => 12,
                    _ => unreachable!(),
                };
                counter[*card as usize] += 1;
            }

            let jokers = counter[0];
            let mut five = false;
            let mut four = false;
            let mut three = false;
            let mut two = 0;
            for count in &counter[1..] {
                match count {
                    5 => five = true,
                    4 => four = true,
                    3 => three = true,
                    2 => two += 1,
                    0 | 1 => {}
                    _ => unreachable!(),
                }
            }

            let kind = if five || four && jokers == 1 {
                FiveOfAKind
            } else if four {
                FourOfAKind
            } else if three {
                match (two, jokers) {
                    (0, 2) => FiveOfAKind,
                    (0, 1) => FourOfAKind,
                    (1, 0) => FullHouse,
                    (0, 0) => ThreeOfAKind,
                    _ => unreachable!(),
                }
            } else {
                match (two, jokers) {
                    (0, 0) => HighCard,
                    (0, 1) => OnePair,
                    (0, 2) => ThreeOfAKind,
                    (0, 3) => FourOfAKind,
                    (0, 4) => FiveOfAKind,
                    (0, 5) => FiveOfAKind,
                    (1, 0) => OnePair,
                    (1, 1) => ThreeOfAKind,
                    (1, 2) => FourOfAKind,
                    (1, 3) => FiveOfAKind,
                    (2, 0) => TwoPair,
                    (2, 1) => FullHouse,
                    _ => unreachable!(),
                }
            };

            Hand { cards, kind, bid }
        })
        .collect();

    solve(hands)
}
