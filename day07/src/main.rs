use std::{cell::RefCell, collections::HashMap};

peg::parser! {
    grammar input_parser() for str {
        rule id() -> &'input str
            = id:$(['a'..='z' | 'A'..='Z' | '0'..='9']+) { id }

        pub(crate) rule gate() -> Gate<'input>
            = lhs:id() " -> " dest:id() {
                Gate::new(lhs, "", Op::Assign, dest)
            }
            / lhs:id() " AND " rhs:id() " -> " dest:id() {
                Gate::new(lhs, rhs, Op::And, dest)
            }
            / lhs:id() " OR " rhs:id() " -> " dest:id() {
                Gate::new(lhs, rhs, Op::Or, dest)
            }
            / lhs:id() " LSHIFT " rhs:id() " -> " dest:id() {
                Gate::new(lhs, rhs, Op::Lshift, dest)
            }
            / lhs:id() " RSHIFT " rhs:id() " -> " dest:id() {
                Gate::new(lhs, rhs, Op::Rshift, dest)
            }
            / "NOT " lhs:id() " -> " dest:id() {
                Gate::new(lhs, "", Op::Not, dest)
            }
    }
}

fn parse_input(input: &str) -> HashMap<&str, RefCell<Gate>> {
    input
        .lines()
        .map(input_parser::gate)
        .map(Result::unwrap)
        .map(|g| (g.dest, RefCell::new(g)))
        .collect()
}

#[derive(Debug, PartialEq)]
enum Op {
    Assign,
    And,
    Or,
    Lshift,
    Rshift,
    Not,
}

struct Gate<'a> {
    lhs: &'a str,
    rhs: &'a str,
    op: Op,
    dest: &'a str,
    value: Option<u16>,
}

impl<'a> Gate<'a> {
    fn new(lhs: &'a str, rhs: &'a str, op: Op, dest: &'a str) -> Self {
        Self {
            lhs,
            rhs,
            op,
            dest,
            value: None,
        }
    }
}

// Find the value of the indentifier `ident` in `curcuit`.
fn value(circuit: &HashMap<&str, RefCell<Gate<'_>>>, ident: &str) -> u16 {
    if let Ok(val) = ident.parse::<u16>() {
        return val;
    }

    let mut gate = circuit.get(ident).unwrap().borrow_mut();
    if let Some(value) = gate.value {
        return value;
    }

    let value = match gate.op {
        Op::Assign => value(circuit, gate.lhs),
        Op::And => value(circuit, gate.lhs) & value(circuit, gate.rhs),
        Op::Or => value(circuit, gate.lhs) | value(circuit, gate.rhs),
        Op::Lshift => value(circuit, gate.lhs) << value(circuit, gate.rhs),
        Op::Rshift => value(circuit, gate.lhs) >> value(circuit, gate.rhs),
        Op::Not => !value(circuit, gate.lhs),
    };

    gate.value = Some(value);
    value
}

fn part1(input: &str) -> u16 {
    let circuit = parse_input(input);
    value(&circuit, "a")
}

fn part2(input: &str) -> u16 {
    let circuit = parse_input(input);
    circuit.get("b").unwrap().borrow_mut().value = Some(part1(input));
    value(&circuit, "a")
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i
    "};

    #[test]
    fn test_value() {
        let circuit = parse_input(SAMPLE);
        assert_eq!(value(&circuit, "d"), 72);
        assert_eq!(value(&circuit, "e"), 507);
        assert_eq!(value(&circuit, "f"), 492);
        assert_eq!(value(&circuit, "g"), 114);
        assert_eq!(value(&circuit, "h"), 65412);
        assert_eq!(value(&circuit, "i"), 65079);
        assert_eq!(value(&circuit, "x"), 123);
        assert_eq!(value(&circuit, "y"), 456);
    }

    #[test]
    fn test_parser() {
        let tests = [
            ("123 -> x", Op::Assign),
            ("x AND y -> d", Op::And),
            ("x OR y -> e", Op::Or),
            ("x LSHIFT 2 -> f", Op::Lshift),
            ("y RSHIFT 2 -> g", Op::Rshift),
            ("NOT x -> h", Op::Not),
        ];
        for (input, expect) in tests {
            let gate = input_parser::gate(input).unwrap();
            assert_eq!(gate.op, expect);
        }
    }
}
