use inputs::read_in_file;
pub fn day02() {
    println!("Day 02");
    let input = read_in_file("./day02/src/day02.txt");
    let part1 = day_02_part_1(&input);
    let part2 = day_02_part_2(&input);
    println!("Part 1: {},\tPart 2: {}", part1, part2);
}

fn day_02_part_1(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let plays: Vec<&str> = line.split(" ").take(2).collect();
        score += match plays[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };
        score += match (plays[0], plays[1]) {
            ("A", "X") => 3,
            ("A", "Y") => 6,
            ("A", "Z") => 0,
            ("B", "X") => 0,
            ("B", "Y") => 3,
            ("B", "Z") => 6,
            ("C", "X") => 6,
            ("C", "Y") => 0,
            ("C", "Z") => 3,
            (_, _) => 0,
        };
    }

    return score;
}

fn day_02_part_2(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let plays: Vec<&str> = line.split(" ").take(2).collect();
        score += match (plays[0], plays[1]) {
            ("A", "X") => 0 + 3,
            ("A", "Y") => 3 + 1,
            ("A", "Z") => 6 + 2,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 0 + 2,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 6 + 1,
            (_, _) => 0,
        };
    }

    return score;
}
#[cfg(test)]
mod tests {
    use crate::{day_02_part_1, day_02_part_2};

    const INPUT1: &str = "A Y\nB X\nC Z";
    const INPUT2: &str = "A Y\nB X\nC Y";
    #[test]
    fn part1_test() {
        let result = day_02_part_1(INPUT1);
        assert_eq!(15, result);
    }
    #[test]
    fn part_1_test_2() {
        let result = day_02_part_1(INPUT2);
        assert_eq!(5 + 6 + 0 + 0, result);
    }
    #[test]
    fn part2_test() {
        let result = day_02_part_2(INPUT1);
        assert_eq!(12, result);
    }
    #[test]
    fn part2_test_2() {
        let result = day_02_part_2(&INPUT2);
        assert_eq!(3 + 1 + 0 + 1 + 3 + 3, result);
    }
}
