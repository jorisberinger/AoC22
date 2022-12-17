use inputs::read_in_file;
use regex::Regex;
use results::print_result;
use std::str::FromStr;
pub fn day10() {
    let input = read_in_file("./day10/src/day10.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(10, result_1, result_2);
}
fn part1(input: &str) -> i32 {
    let cycles = get_cycles(input);
    return (20..=220)
        .step_by(40)
        .map(|x| x * cycles[x as usize - 1])
        .sum();
}

fn part2(input: &str) -> &str {
    let cycles = get_cycles(input);
    for row in 0..=220 / 40 {
        for i in 0..40 {
            if (cycles[i + row * 40] - i as i32).abs() <= 1 {
                // print!("x")
            } else {
                // print!(".")
            }
        }
        // println!()
    }
    "ZRARLFZU"
}
fn get_cycles(input: &str) -> Vec<i32> {
    let mut cycles: Vec<i32> = Vec::from([1]);
    let noop = Regex::new(r"noop").unwrap();
    let add = Regex::new(r"addx ([-\d]+)").unwrap();
    for line in input.lines() {
        let last = cycles.last().unwrap().to_owned();
        if noop.is_match(line) {
            cycles.push(last)
        }
        if add.is_match(line) {
            let caps = add.captures(line).unwrap();
            cycles.push(last);

            let new_value: i32 = FromStr::from_str(caps.get(1).unwrap().as_str()).unwrap();
            cycles.push(last + new_value);
        }
    }
    cycles
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let input = read_in_file("./src/test.txt");
        let result = part1(&input);
        assert_eq!(13140, result)
    }
    #[test]
    fn part2_test() {
        let input = read_in_file("./src/test.txt");
        let result = part2(&input);
    }
}
