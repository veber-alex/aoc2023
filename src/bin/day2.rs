fn main() {
    let input = include_str!("../../input/day2.txt");

    assert_eq!(part1(input), 3035);
    assert_eq!(part2(input), 66027);
}

#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse(input: &str) -> Vec<Vec<CubeSet>> {
    let mut games = vec![];
    for line in input.lines() {
        let (_, data) = line.split_once(":").unwrap();
        let mut game = vec![];
        for set in data.split(";") {
            let mut cube_set = CubeSet::default();
            for color in set.split(",") {
                let (n, name) = color.trim().split_once(" ").unwrap();
                let n = n.parse().unwrap();
                match name {
                    "red" => cube_set.red = n,
                    "green" => cube_set.green = n,
                    "blue" => cube_set.blue = n,
                    _ => unreachable!(),
                }
            }
            game.push(cube_set);
        }
        games.push(game);
    }

    games
}

fn part1(input: &str) -> u32 {
    let games = parse(input);
    let mut sum = 0;
    'game: for (game, id) in games.iter().zip(1..) {
        for set in game {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                continue 'game;
            }
        }
        sum += id;
    }

    sum
}

fn part2(input: &str) -> u32 {
    let games = parse(input);
    let mut sum = 0;
    for game in games {
        let min_red = game.iter().map(|set| set.red).max().unwrap();
        let min_green = game.iter().map(|set| set.green).max().unwrap();
        let min_blue = game.iter().map(|set| set.blue).max().unwrap();
        sum += min_red * min_green * min_blue;
    }

    sum
}
