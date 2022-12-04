use inputs::read_in_file;
use regex::Captures;
use regex::Regex;
use results::print_result;
pub fn day04() {
    let input = read_in_file("./day04/src/day04.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(04, result_1, result_2);
}
fn part1(input: &str) -> i32 {
    let mut result = 0;
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let s1 = getMatch(&captures, 1);
        let s2 = getMatch(&captures, 2);
        let s3 = getMatch(&captures, 3);
        let s4 = getMatch(&captures, 4);

        if (s1 <= s3 && s2 >= s4) || (s3 <= s1 && s4 >= s2) {
            result += 1;
        }
    }
    return result;
}

fn getMatch(captures: &Captures, i: usize) -> i32 {
    captures
        .get(i)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .unwrap()
}
fn part2(input: &str) -> i32 {
    let mut result = 0;
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let s1 = getMatch(&captures, 1);
        let s2 = getMatch(&captures, 2);
        let s3 = getMatch(&captures, 3);
        let s4 = getMatch(&captures, 4);

        if s1 <= s3 && s2 >= s3 || s3 <= s1 && s4 >= s1 {
            result += 1;
        }
    }
    return result;
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(2, result)
    }
    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(4, result)
    }
}
