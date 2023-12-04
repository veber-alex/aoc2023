fn main() {
    let input = include_str!("../../input/day4.txt");

    assert_eq!(part1(input), 22193);
    assert_eq!(part2(input), 5625994);
}

#[derive(Debug)]
struct Card {
    score: usize,
    copies: u32,
}

fn parse(input: &str) -> Vec<Card> {
    let mut winning_numbers = vec![];
    input
        .lines()
        .map(|line| {
            let (_, data) = line.split_once(':').unwrap();
            let (win, nums) = data.split_once('|').unwrap();
            winning_numbers.extend(win.trim().split_ascii_whitespace());
            let score = nums
                .trim()
                .split_ascii_whitespace()
                .filter(|n| winning_numbers.contains(n))
                .count();
            winning_numbers.clear();
            Card { score, copies: 1 }
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    parse(input)
        .iter()
        .map(|card| card.score.checked_sub(1).map(|n| 1 << n).unwrap_or(0))
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut cards = parse(input);
    for idx in 0..cards.len() {
        let card = &cards[idx];
        let card_copies = card.copies;
        for next in 1..=card.score {
            cards[idx + next].copies += card_copies
        }
    }

    cards.iter().map(|card| card.copies).sum()
}
