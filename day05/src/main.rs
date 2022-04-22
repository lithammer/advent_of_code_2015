use fancy_regex::Regex;

fn is_nice(word: &str) -> bool {
    let mut num_vowels = 0;
    let mut has_dupe = false;

    let mut chars = word.chars().peekable();
    while let Some(curr) = chars.next() {
        if let Some(&next) = chars.peek() {
            if matches!(
                (curr, next),
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')
            ) {
                return false;
            }
            if curr == next {
                has_dupe = true;
            }
        }
        if matches!(curr, 'a' | 'e' | 'i' | 'o' | 'u') {
            num_vowels += 1;
        }
    }

    num_vowels >= 3 && has_dupe
}

fn is_really_nice(word: &str) -> bool {
    // Non-overlapping pairs.
    let cond1 = Regex::new(r"(..).*\1").unwrap();
    // Repeated letters with exactly on letter in between.
    let cond2 = Regex::new(r"(.).\1").unwrap();

    cond1.is_match(word).unwrap_or(false) && cond2.is_match(word).unwrap_or(false)
}

fn part1(input: &str) -> usize {
    input.lines().filter(|word| is_nice(word)).count()
}

fn part2(input: &str) -> usize {
    input.lines().filter(|word| is_really_nice(word)).count()
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
    fn test_is_nice_true() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
    }

    #[test]
    fn test_is_nice_false() {
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_nice_improved_true() {
        assert!(is_really_nice("qjhvhtzxzqqjkmpb"));
        assert!(is_really_nice("xxyxx"));
    }

    #[test]
    fn test_is_nice_improved_false() {
        assert!(!is_really_nice("uurcxstgmygtbstg"));
        assert!(!is_really_nice("ieodomkazucvgmuy"));
    }
}
