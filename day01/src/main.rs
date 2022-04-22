enum Token {
    Up,
    Down,
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '(' => Token::Up,
            ')' => Token::Down,
            _ => unreachable!(),
        }
    }
}

fn part1(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| match c.into() {
        Token::Up => acc + 1,
        Token::Down => acc - 1,
    })
}

fn part2(input: &str) -> i32 {
    // let mut floor = 0;
    // for (i, c) in input.chars().enumerate() {
    //     match c.into() {
    //         Token::Up => floor += 1,
    //         Token::Down => floor -= 1,
    //     };

    //     if floor == -1 {
    //         return (i + 1).try_into().unwrap();
    //     }
    // }
    // floor

    input
        .chars()
        .enumerate()
        .try_fold(0, |acc, (i, c)| {
            let floor = match c.into() {
                Token::Up => acc + 1,
                Token::Down => acc - 1,
            };

            match floor {
                -1 => Err((i + 1).try_into().unwrap()),
                _ => Ok(floor),
            }
        })
        .unwrap_or_else(|e| e)
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
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }
}
