use std::collections::HashSet;

use inputs::read_in_file;
use results::print_result;
pub fn day06() {
    let input = read_in_file("./day06/src/day06.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(6, result_1, result_2);
}
fn part1(input: &str) -> i32 {
    if let Some(value) = find_marker(input, 4) {
        return value;
    }
    panic!("No start marker found");
}

fn find_marker(input: &str, length_of_marker: usize) -> Option<i32> {
    let line = input.lines().next().unwrap();
    let chars = line.chars().collect::<Vec<char>>();
    for (i, window) in chars.windows(length_of_marker).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window);
        if set.len() == length_of_marker {
            return Some((i + length_of_marker).try_into().unwrap());
        }
    }
    panic!("No start marker found");
}
fn part2(input: &str) -> i32 {
    if let Some(value) = find_marker(input, 14) {
        return value;
    }
    panic!("No start marker found");
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    #[test]
    fn part1_test() {
        assert_eq!(7, part1(INPUT));
        assert_eq!(5, part1(INPUT1));
        assert_eq!(6, part1(INPUT2));
        assert_eq!(10, part1(INPUT3));
        assert_eq!(11, part1(INPUT4));
    }
    #[test]
    fn part2_test() {
        assert_eq!(19, part2(INPUT));
        assert_eq!(23, part2(INPUT1));
        assert_eq!(23, part2(INPUT2));
        assert_eq!(29, part2(INPUT3));
        assert_eq!(26, part2(INPUT4));
    }
}
