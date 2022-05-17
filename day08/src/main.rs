fn score_string(s: &str) -> i32 {
    let mut chars = s.chars();
    let mut score = 0;

    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some(c2) if c2 == 'x' => {
                    chars.next();
                    chars.next();
                    score += 3;
                }
                Some(_) => {
                    score += 1;
                }
                None => panic!("unexpected end of string: {}", s),
            },
            '"' => {
                score += 1;
            }
            _ => {}
        }
    }

    score
}

fn part1(input: &str) -> i32 {
    input.lines().map(score_string).sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 = {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const SAMPLE: &str = indoc! {r#"
        ""
        "abc"
        "aaa\"aaa"
        "\x27"
    "#};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 12);
    }

    #[test]
    fn test_score_string() {
        for (string, score) in SAMPLE.lines().zip([2, 2, 3, 5]) {
            assert_eq!(score_string(string), score);
        }
    }
}
