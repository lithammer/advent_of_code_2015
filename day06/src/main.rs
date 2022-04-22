peg::parser! {
    grammar input_parser() for str {
        rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule point() -> Point
            = a:number() "," b:number() { (a, b) }

        rule action() -> Action
            = "toggle" { Action::Toggle }
            / "turn off" { Action::Off }
            / "turn on" { Action::On }

        // toggle 780,318 through 975,495
        // turn off 185,412 through 796,541
        // turn on 879,548 through 892,860
        pub(crate) rule instruction() -> Instruction
            = a:action() " " p1:point() " through " p2:point() {
                Instruction { coords: (p1, p2), action: a }
            }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input
        .lines()
        .map(input_parser::instruction)
        .map(Result::unwrap)
}

type Point = (usize, usize);

#[derive(Debug, PartialEq)]
struct Instruction {
    coords: (Point, Point),
    action: Action,
}

impl Instruction {
    fn iter_coords(&self) -> impl Iterator<Item = Point> {
        let (p1, p2) = self.coords;
        let x1 = p1.0.min(p2.0);
        let x2 = p1.0.max(p2.0);
        let y1 = p1.1.min(p2.1);
        let y2 = p1.1.max(p2.1);
        (y1..=y2).flat_map(move |y| (x1..=x2).map(move |x| (x as usize, y as usize)))
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    On,
    Off,
    Toggle,
}

fn part1(input: &str) -> u32 {
    let mut lights = [[0u8; 1000]; 1000];

    for instruction in parse_input(input) {
        let apply = match instruction.action {
            Action::On => |n: &mut u8| *n = 1,
            Action::Off => |n: &mut u8| *n = 0,
            Action::Toggle => |n: &mut u8| *n ^= 0x1,
        };

        for (x, y) in instruction.iter_coords() {
            apply(&mut lights[y][x])
        }
    }

    lights.iter().flatten().map(|&x| x as u32).sum()
}

fn part2(input: &str) -> u32 {
    let mut lights = [[0u16; 1000]; 1000];

    for instruction in parse_input(input) {
        let apply = match instruction.action {
            Action::On => |n: &mut u16| *n += 1,
            Action::Off => |n: &mut u16| {
                if *n > 0 {
                    *n -= 1
                }
            },
            Action::Toggle => |n: &mut u16| *n += 2,
        };

        for (x, y) in instruction.iter_coords() {
            apply(&mut lights[y][x])
        }
    }

    lights.iter().flatten().map(|&x| x as u32).sum()
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
    fn test_part1() {}

    #[test]
    fn test_parse_input() {
        let tests = [
            (
                "toggle 780,318 through 975,495",
                Instruction {
                    coords: ((780, 318), (975, 495)),
                    action: Action::Toggle,
                },
            ),
            (
                "turn off 185,412 through 796,541",
                Instruction {
                    coords: ((185, 412), (796, 541)),
                    action: Action::Off,
                },
            ),
            (
                "turn on 879,548 through 892,860",
                Instruction {
                    coords: ((879, 548), (892, 860)),
                    action: Action::On,
                },
            ),
        ];
        for (input, expect) in tests {
            assert_eq!(input_parser::instruction(input).unwrap(), expect);
        }
    }
}
