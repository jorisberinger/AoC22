use inputs::read_in_file;
use results::print_result;
use std::{collections::HashMap, str::FromStr};
pub fn day08() {
    let input = read_in_file("./day08/src/day08.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(8, result_1, result_2);
}
fn part1(input: &str) -> i32 {
    let mut trees: HashMap<(usize, usize), u8> = HashMap::new();
    let mut length = 0;
    for (x, line) in input.lines().enumerate() {
        length += 1;
        for (y, char) in line.chars().enumerate() {
            trees.insert((x, y), FromStr::from_str(&char.to_string()).unwrap());
        }
    }
    let mut visible_trees = length as i32 * 4 - 4;
    for x in 1..(length - 1) {
        for y in 1..(length - 1) {
            let tree_size = trees.get(&(x, y)).unwrap();
            let mut is_visible = false;
            if (0..x).all(|i| tree_size > trees.get(&(i, y)).unwrap())
                || (x + 1..length).all(|i| tree_size > trees.get(&(i, y)).unwrap())
                || (0..y).all(|i| tree_size > trees.get(&(x, i)).unwrap())
                || (y + 1..length).all(|i| tree_size > trees.get(&(x, i)).unwrap())
            {
                visible_trees += 1;
            }
        }
    }
    return visible_trees;
}
fn part2(input: &str) -> i32 {
    return 0;
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373
25512
65332
33549
35390";
    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(21, result)
    }
    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(0, result)
    }
}
