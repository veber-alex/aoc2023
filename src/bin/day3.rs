use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    BTreeMap, HashMap,
};

fn main() {
    let input = include_str!("../../input/day3.txt");

    assert_eq!(part1(input), 512794);
    assert_eq!(part2(input), 67779080);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
struct Pos {
    y: i32,
    x: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn parse(input: &str) -> BTreeMap<Pos, char> {
    input
        .lines()
        .zip(0..)
        .flat_map(|(line, y)| {
            line.chars()
                .zip(0..)
                .map(move |(sym, x)| (Pos::new(x, y), sym))
        })
        .collect()
}

fn look_around(map: &BTreeMap<Pos, char>, pos: Pos) -> Option<(Pos, char)> {
    let Pos { x, y } = pos;
    [
        Pos::new(x + 1, y),
        Pos::new(x + 1, y + 1),
        Pos::new(x + 1, y - 1),
        Pos::new(x, y + 1),
        Pos::new(x, y - 1),
        Pos::new(x - 1, y + 1),
        Pos::new(x - 1, y),
        Pos::new(x - 1, y - 1),
    ]
    .iter()
    .filter_map(|pos| map.get_key_value(pos).map(|(k, v)| (*k, *v)))
    .find(|(_, sym)| !matches!(sym, '.' | '0'..='9'))
}

fn part1(input: &str) -> u32 {
    let map = parse(input);
    let mut buf = String::new();
    let mut sum = 0;
    let mut has_sym = false;

    for (&pos, &c) in map.iter() {
        if c.is_ascii_digit() {
            buf.push(c);
            has_sym |= look_around(&map, pos).is_some();
        } else if !buf.is_empty() {
            if has_sym {
                let number: u32 = buf.parse().unwrap();
                sum += number;
                has_sym = false;
            }
            buf.clear();
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let map = parse(input);
    let mut buf = String::new();
    let mut sum = 0;
    let mut star_pos = None;
    let mut count = HashMap::new();

    for (&pos, &c) in map.iter() {
        if c.is_ascii_digit() {
            buf.push(c);
            if let Some((p, '*')) = look_around(&map, pos) {
                star_pos = Some(p);
            }
        } else if !buf.is_empty() {
            if let Some(p) = star_pos {
                let number: u32 = buf.parse().unwrap();
                match count.entry(p) {
                    Occupied(entry) => {
                        sum += number * entry.get();
                    }
                    Vacant(entry) => {
                        entry.insert(number);
                    }
                }
                star_pos = None;
            }
            buf.clear();
        }
    }

    sum
}
