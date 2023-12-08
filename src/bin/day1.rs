fn main() {
    let input = include_str!("../../input/day1.txt");

    assert_eq!(part1(input), 54630);
    assert_eq!(part2(input), 54770);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line.bytes().find(u8::is_ascii_digit).unwrap() as u32;
            let last = line.bytes().rev().find(u8::is_ascii_digit).unwrap() as u32;
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

fn scan_line(line: &str) -> u32 {
    let mut min_index = line.len();
    let mut min_value = 0;
    let mut max_index = 0;
    let mut max_value = 0;

    for ((num, name), idx) in MAP.iter().copied().zip(1..) {
        for (pos, _) in line.match_indices(num).chain(line.match_indices(name)) {
            if pos < min_index {
                min_index = pos;
                min_value = idx;
            }
            if pos >= max_index {
                max_index = pos;
                max_value = idx;
            }
        }
    }

    min_value * 10 + max_value
}
