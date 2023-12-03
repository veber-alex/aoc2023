fn main() {
    let input = include_str!("../../input/day1.txt");

    assert_eq!(part1(input), 54630);
    assert_eq!(part2(input), 54770);
}

fn part1(input: &str) -> u32 {
    let find = |b: &u8| matches!(b, b'1'..=b'9');
    input
        .lines()
        .map(|line| {
            let first = line.bytes().find(find).unwrap() as u32;
            let last = line.bytes().rev().find(find).unwrap() as u32;
            (first - 48) * 10 + (last - 48)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input.lines().map(scan_line).sum()
}

const MAP: &[(char, &str)] = &[
    ('1', "one"),
    ('2', "two"),
    ('3', "three"),
    ('4', "four"),
    ('5', "five"),
    ('6', "six"),
    ('7', "seven"),
    ('8', "eight"),
    ('9', "nine"),
];

fn scan_line(mut line: &str) -> u32 {
    let first = 'out: loop {
        for ((num, name), idx) in MAP.iter().copied().zip(1..) {
            if line.starts_with(num) || line.starts_with(name) {
                break 'out idx;
            }
        }
        line = &line[1..];
    };

    let last = 'out: loop {
        for ((num, name), idx) in MAP.iter().copied().zip(1..) {
            if line.ends_with(num) || line.ends_with(name) {
                break 'out idx;
            }
        }
        line = &line[..line.len() - 1];
    };

    first * 10 + last
}
