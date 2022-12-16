use std::collections::HashMap;

use inputs::read_in_file;
use regex::Regex;
use results::print_result;
use std::str::FromStr;
pub fn day07() {
    let input = read_in_file("./day07/src/day07.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(7, result_1, result_2);
}
fn part1(input: &str) -> i32 {
    let mut current_path: Vec<String> = Vec::new();
    let mut sizes: HashMap<String, i32> = HashMap::new();
    let lines = input.lines();
    let cmd_cd = Regex::new(r"\$ cd ([/a-zA-Z\.]+)").unwrap();
    let cmd_cd_up = Regex::new(r"\$ cd \.\.").unwrap();
    let cmd_ls = Regex::new(r"\$ ls").unwrap();
    let file = Regex::new(r"(\d+) [a-zA-Z]+").unwrap();
    let dir = Regex::new(r"dir ([a-zA-Z]+)").unwrap();
    for line in lines {
        if cmd_cd_up.is_match(line) {
            current_path.pop();
            continue;
        }
        if cmd_cd.is_match(line) {
            let caps = cmd_cd.captures(line).unwrap();
            current_path.push(current_path.join("-") + caps.get(1).unwrap().as_str());
        }
        if cmd_ls.is_match(line) {}

        if file.is_match(line) {
            let caps = file.captures(line).unwrap();

            for path in &current_path {
                let size = sizes.get(path).unwrap_or(&0);
                let file_size: i32 = FromStr::from_str(caps.get(1).unwrap().as_str()).unwrap();
                sizes.insert(path.to_string(), size + file_size);
            }
        }
        if dir.is_match(line) {
            let caps = dir.captures(line).unwrap();
        }
    }
    return sizes.values().filter(|x| **x <= 100000).sum();
}
fn part2(input: &str) -> i32 {
    return 0;
}
#[cfg(test)]
mod tests {
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    use super::*;
    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(95437, result)
    }
    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(0, result)
    }
}
