use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/day8.txt");
    let (directions, nodes) = parse(input);
    assert_eq!(part1(directions, &nodes, "AAA"), 14681);
    assert_eq!(part2(directions, &nodes), 14321394058031);
}

fn parse(input: &'static str) -> (&'static str, HashMap<&'static str, [&'static str; 2]>) {
    let (directions, nodes) = input.split_once("\n\n").unwrap();
    let nodes = nodes
        .lines()
        .map(|line| (&line[0..3], [&line[7..10], &line[12..15]]))
        .collect();

    (directions, nodes)
}

fn part1(
    directions: &'static str,
    nodes: &HashMap<&'static str, [&'static str; 2]>,
    mut start: &str,
) -> u64 {
    for (dir, i) in directions.bytes().cycle().zip(1..) {
        start = &nodes[start][(dir == b'R') as usize];
        if start.ends_with('Z') {
            return i;
        }
    }

    unreachable!()
}

fn part2(directions: &'static str, nodes: &HashMap<&'static str, [&'static str; 2]>) -> u64 {
    let starts = nodes.keys().copied().filter(|node| node.ends_with('A'));

    find_lcm(starts.map(|pos| part1(directions, nodes, pos)))
}

fn find_lcm(nums: impl Iterator<Item = u64>) -> u64 {
    fn lcm(num1: u64, num2: u64) -> u64 {
        let (mut num, mut den) = if num1 > num2 {
            (num1, num2)
        } else {
            (num2, num1)
        };
        let mut rem = num % den;
        while rem != 0 {
            num = den;
            den = rem;
            rem = num % den;
        }
        num1 * num2 / den
    }

    nums.reduce(lcm).unwrap()
}
