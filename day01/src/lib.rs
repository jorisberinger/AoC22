use inputs::read_in_file;
use results::print_result;
use std::cmp::max;
pub fn day01() {
    let input = read_in_file("./day01/src/day01.txt");
    let result_part_1 = day_01_part_1(&input);
    let result_part_2 = day_01_part_2(&input);
    print_result(1, result_part_1, result_part_2)
}

fn day_01_part_1(input: &str) -> i32 {
    let lines = input.lines();
    let mut max_calories = 0;
    let mut current = 0;

    for line in lines {
        if line.is_empty() {
            max_calories = max(max_calories, current);
            current = 0;
            continue;
        }

        let num = line.parse::<i32>();
        current += num.unwrap()
    }
    return max_calories;
}

fn day_01_part_2(input: &str) -> i32 {
    let lines = input.lines();
    let mut elves = Vec::new();
    let mut current = 0;
    for line in lines {
        if line.is_empty() {
            elves.push(current);
            current = 0;
            continue;
        }
        let num = line.parse::<i32>();
        current += num.unwrap();
    }
    elves.push(current);
    elves.sort();
    return elves.iter().rev().take(3).sum();
}
#[cfg(test)]
mod tests {
    use crate::{day_01_part_1, day_01_part_2};

    const INPUT_1: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    const INPUT_2: &str = "1000
2000
3000

4000

5000
6000

10000";
    #[test]
    fn day_01_part_1_test() {
        let result = day_01_part_1(INPUT_1);
        assert_eq!(24000, result)
    }

    #[test]
    fn day_01_part_1_own_test() {
        let result = day_01_part_1(INPUT_2);
        assert_eq!(11000, result)
    }
    #[test]
    fn day_01_part_2_test() {
        let result = day_01_part_2(INPUT_1);
        assert_eq!(45000, result)
    }

    #[test]
    fn day_01_part_2_own_test() {
        let result = day_01_part_2(INPUT_2);
        assert_eq!(27000, result)
    }
}
