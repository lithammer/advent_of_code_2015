use base16ct::lower::encode_str;
use md5::{Digest, Md5};

fn mine_advent_coin(secret_key: &[u8], query: &str) -> u32 {
    let mut buf = [0u8; 32];
    let mut hasher = Md5::new();
    let mut n = 0u32;

    loop {
        hasher.update(secret_key);
        hasher.update(n.to_string());
        let digest = hasher.finalize_reset();
        let hex_digest = encode_str(&digest, &mut buf).unwrap();
        if hex_digest.starts_with(query) {
            break;
        }
        n += 1;
    }

    n
}

fn part1(input: &str) -> u32 {
    let secret_key = input.trim().as_bytes();
    mine_advent_coin(secret_key, "00000")
}

fn part2(input: &str) -> u32 {
    let secret_key = input.trim().as_bytes();
    mine_advent_coin(secret_key, "000000")
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
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
