use std::collections::{HashMap, HashSet};

use inputs::read_in_file;
use results::print_result;
pub fn day03() {
    let input = read_in_file("./day03/src/day03.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(3, result_1, result_2);
}

fn part1(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let item = get_wrong_item(line);
        let score = get_item_score(item);
        result += score;
    }
    return result;
}

fn part2(input: &str) -> i32 {
    let mut result = 0;
    let mut items: HashSet<char> = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        let chars: HashSet<char> = line.chars().collect();
        match i % 3 {
            0 => items = chars,
            1 => items = items.intersection(&chars).map(|c| c.clone()).collect(),
            2 => {
                let badge = items
                    .intersection(&chars)
                    .map(|c| c.clone())
                    .collect::<Vec<char>>()[0];
                result += get_item_score(badge);
            }
            _ => panic!("Math is wrong"),
        }
    }
    return result;
}
fn get_wrong_item(line: &str) -> char {
    let mut chars = HashSet::new();
    for (i, char) in line.chars().enumerate() {
        if chars.contains(&char) && i >= line.len() / 2 {
            return char;
        }
        if i < line.len() / 2 {
            chars.insert(char.clone());
        }
    }
    panic!("must have found a char")
}

fn get_item_score(item: char) -> i32 {
    let alphabet: HashMap<char, i32> = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .zip(1i32..)
        .map(|(c, i)| (c as char, i))
        .collect();

    return alphabet.get(&item).unwrap().clone();
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(157, result)
    }

    #[test]
    fn get_wrong_item_test() {
        let wrong_item = get_wrong_item("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!('p', wrong_item);
        assert_eq!('L', get_wrong_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"))
    }

    #[test]
    fn get_item_score_test() {
        assert_eq!(1, get_item_score('a'));
        assert_eq!(16, get_item_score('p'));
        assert_eq!(42, get_item_score('P'));
    }
    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(70, result);
    }
}
