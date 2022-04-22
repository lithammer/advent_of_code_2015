use std::str::FromStr;

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn area(&self) -> u32 {
        (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length)
    }
}

impl FromStr for Present {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('x').collect::<Vec<_>>();
        Ok(Self {
            length: parts[0].parse()?,
            width: parts[1].parse()?,
            height: parts[2].parse()?,
        })
    }
}

fn paper_needed(b: Present) -> u32 {
    let smallest_side = (b.length * b.width)
        .min(b.width * b.height)
        .min(b.height * b.length);
    b.area() + smallest_side
}

fn ribbon_needed(b: Present) -> u32 {
    let mut dimensions = [b.length, b.width, b.height];
    dimensions.sort_unstable();
    let bow = dimensions.into_iter().reduce(|acc, x| acc * x).unwrap_or(0);
    let ribbon = dimensions.into_iter().take(2).map(|x| x * 2).sum::<u32>();
    ribbon + bow
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(str::parse::<Present>)
        .map(Result::unwrap)
        .map(paper_needed)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(str::parse::<Present>)
        .map(Result::unwrap)
        .map(ribbon_needed)
        .sum()
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
        assert_eq!(part1("2x3x4"), 58);
        assert_eq!(part1("1x1x10"), 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("2x3x4"), 34);
        assert_eq!(part2("1x1x10"), 14);
    }

    #[test]
    fn test_box_from_str() -> Result<(), std::num::ParseIntError> {
        let b: Present = "2x3x4".parse()?;
        assert_eq!(b.length, 2);
        assert_eq!(b.width, 3);
        assert_eq!(b.height, 4);
        Ok(())
    }

    #[test]
    fn test_box_area() {
        let b = Present {
            length: 2,
            width: 3,
            height: 4,
        };
        assert_eq!(b.area(), 52);
    }
}
