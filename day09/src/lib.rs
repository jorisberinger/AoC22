use inputs::read_in_file;
use regex::Regex;
use results::print_result;
use std::collections::HashSet;
pub fn day09() {
    let input = read_in_file("./day09/src/day09.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(9, result_1, result_2);
}
fn part1(input: &str) -> i32 {
    let mut visited_places: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut head_position: (i32, i32) = (0, 0);
    let mut tail_position: (i32, i32) = (0, 0);
    let reg = Regex::new(r"([RULD]) (\d+)").unwrap();
    for line in input.lines() {
        if let Some(caps) = reg.captures(line) {
            let direction = caps.get(1).unwrap().as_str();
            let steps: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            // println!("{:?} {:?}", direction, steps);
            for _ in 0..steps {
                head_position = match direction {
                    "R" => (head_position.0 + 1, head_position.1),
                    "U" => (head_position.0, head_position.1 + 1),
                    "L" => (head_position.0 - 1, head_position.1),
                    "D" => (head_position.0, head_position.1 - 1),
                    &_ => todo!(""),
                };
                // println!("new head position: {:?}", head_position);
                update_tail(&mut tail_position, head_position);
                // println!("new tail position: {:?}", tail_position);
                visited_places.insert(tail_position);
            }
        }
    }
    return visited_places.len() as i32;
}

fn part2(input: &str) -> i32 {
    let mut visited_places: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut tail_positions: Vec<(i32, i32)> = vec![(0, 0); 10];
    // println!("tail positions {:?}", tail_positions);
    let reg = Regex::new(r"([RULD]) (\d+)").unwrap();
    for line in input.lines() {
        if let Some(caps) = reg.captures(line) {
            let direction = caps.get(1).unwrap().as_str();
            let steps: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            // println!("{:?} {:?}", direction, steps);
            for _ in 0..steps {
                tail_positions[0] = match direction {
                    "R" => (tail_positions[0].0 + 1, tail_positions[0].1),
                    "U" => (tail_positions[0].0, tail_positions[0].1 + 1),
                    "L" => (tail_positions[0].0 - 1, tail_positions[0].1),
                    "D" => (tail_positions[0].0, tail_positions[0].1 - 1),
                    &_ => todo!(""),
                };

                // println!("new head position: {:?}", head_position);
                update_tail2(&mut tail_positions);
                // println!("new tail position: {:?}", tail_position);
                // println!("step {}, tail {:?}", i, tail_positions);
                visited_places.insert(tail_positions[9]);
            }
        }
    }
    return visited_places.len() as i32;
}
fn update_tail2(tail_positions: &mut Vec<(i32, i32)>) {
    for i in 1..tail_positions.len() {
        // println!(
        //     "t1: {:?}, t2: {:?}",
        //     tail_positions[i - 1],
        //     tail_positions[i]
        // );
        tail_positions[i] = match (
            tail_positions[i - 1].0 - tail_positions[i].0,
            tail_positions[i - 1].1 - tail_positions[i].1,
        ) {
            (2, 0) => (tail_positions[i].0 + 1, tail_positions[i].1),
            (-2, 0) => (tail_positions[i].0 - 1, tail_positions[i].1),
            (0, 2) => (tail_positions[i].0, tail_positions[i].1 + 1),
            (0, -2) => (tail_positions[i].0, tail_positions[i].1 - 1),
            (2, 1) => (tail_positions[i].0 + 1, tail_positions[i].1 + 1),
            (2, -1) => (tail_positions[i].0 + 1, tail_positions[i].1 - 1),
            (-2, 1) => (tail_positions[i].0 - 1, tail_positions[i].1 + 1),
            (-2, -1) => (tail_positions[i].0 - 1, tail_positions[i].1 - 1),
            (1, 2) => (tail_positions[i].0 + 1, tail_positions[i].1 + 1),
            (-1, 2) => (tail_positions[i].0 - 1, tail_positions[i].1 + 1),
            (1, -2) => (tail_positions[i].0 + 1, tail_positions[i].1 - 1),
            (-1, -2) => (tail_positions[i].0 - 1, tail_positions[i].1 - 1),
            (2, 2) => (tail_positions[i].0 + 1, tail_positions[i].1 + 1),
            (2, -2) => (tail_positions[i].0 + 1, tail_positions[i].1 - 1),
            (-2, 2) => (tail_positions[i].0 - 1, tail_positions[i].1 + 1),
            (-2, -2) => (tail_positions[i].0 - 1, tail_positions[i].1 - 1),
            (0, 0) => {
                // println!("Cover");
                tail_positions[i]
            }
            (1, 0) | (0, 1) | (-1, 0) | (0, -1) | (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => {
                // println!("Touching");
                tail_positions[i]
            }
            (_, _) => {
                println!(
                    "NOT TOUCHING NOT MOVING, h {:?} t {:?}",
                    tail_positions[i - 1],
                    tail_positions[i]
                );
                panic!();
            }
        };
    }
    // println!("TAIL {:?}", tail_positions);
}
fn update_tail(tail_position: &mut (i32, i32), head_position: (i32, i32)) {
    *tail_position = match (
        head_position.0 - tail_position.0,
        head_position.1 - tail_position.1,
    ) {
        (2, 0) => (tail_position.0 + 1, tail_position.1),
        (-2, 0) => (tail_position.0 - 1, tail_position.1),
        (0, 2) => (tail_position.0, tail_position.1 + 1),
        (0, -2) => (tail_position.0, tail_position.1 - 1),
        (2, 1) => (tail_position.0 + 1, tail_position.1 + 1),
        (2, -1) => (tail_position.0 + 1, tail_position.1 - 1),
        (-2, 1) => (tail_position.0 - 1, tail_position.1 + 1),
        (-2, -1) => (tail_position.0 - 1, tail_position.1 - 1),
        (1, 2) => (tail_position.0 + 1, tail_position.1 + 1),
        (-1, 2) => (tail_position.0 - 1, tail_position.1 + 1),
        (1, -2) => (tail_position.0 + 1, tail_position.1 - 1),
        (-1, -2) => (tail_position.0 - 1, tail_position.1 - 1),
        (0, 0) => {
            // println!("Cover");
            *tail_position
        }
        (1, 0) | (0, 1) | (-1, 0) | (0, -1) | (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => {
            // println!("Touching");
            *tail_position
        }
        (_, _) => {
            println!(
                "NOT TOUCHING NOT MOVING, h {:?} t {:?}",
                head_position, tail_position
            );
            *tail_position
        }
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(13, result)
    }
    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(1, result)
    }
    #[test]
    fn part2_test_large() {
        let result = part2(INPUT2);
        assert_eq!(36, result)
    }
    #[test]
    fn tail_1() {
        let head_position = (2, 0);
        let mut tail_position = (0, 0);
        update_tail(&mut tail_position, head_position);
        assert_eq!((1, 0), tail_position);
    }
    #[test]
    fn tail_2() {
        let head_position = (2, 1);
        let mut tail_position = (0, 0);
        update_tail(&mut tail_position, head_position);
        assert_eq!((1, 1), tail_position);
    }
    #[test]
    fn tail_3() {
        let head_position = (-1, -2);
        let mut tail_position = (0, 0);
        update_tail(&mut tail_position, head_position);
        assert_eq!((-1, -1), tail_position);
    }
}
