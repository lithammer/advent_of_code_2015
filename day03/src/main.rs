use std::collections::HashSet;

enum Move {
    North,
    South,
    East,
    West,
}

type House = (i32, i32);

fn parse_input(input: &str) -> impl Iterator<Item = Move> + '_ {
    input.chars().map(|c| match c {
        '^' => Move::North,
        'v' => Move::South,
        '>' => Move::East,
        '<' => Move::West,
        _ => unreachable!(),
    })
}

fn part1(input: &str) -> u32 {
    let mut houses = HashSet::<House>::new();
    houses.insert((0, 0));
    parse_input(input).fold((0, 0), |(x, y), m| {
        let (dx, dy) = match m {
            Move::North => (0, 1),
            Move::South => (0, -1),
            Move::East => (1, 0),
            Move::West => (-1, 0),
        };
        houses.insert((x + dx, y + dy));
        (x + dx, y + dy)
    });
    houses.len() as u32
}

fn part2(input: &str) -> u32 {
    let mut houses = HashSet::<House>::new();
    houses.insert((0, 0));
    parse_input(input)
        .enumerate()
        .fold(((0, 0), (0, 0)), |((x1, y1), (x2, y2)), (i, m)| {
            let (dx, dy) = match m {
                Move::North => (0, 1),
                Move::South => (0, -1),
                Move::East => (1, 0),
                Move::West => (-1, 0),
            };
            if i % 2 == 0 {
                // Santa's move.
                houses.insert((x1 + dx, y1 + dy));
                ((x1 + dx, y1 + dy), (x2, y2))
            } else {
                // Robo-Santa's move.
                houses.insert((x2 + dx, y2 + dy));
                ((x1, y1), (x2 + dx, y2 + dy))
            }
        });

    houses.len() as u32
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("^v"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}
