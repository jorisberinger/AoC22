use inputs::read_in_file;
use log::{debug, info};
use results::print_result;

use serde::{Deserialize, Serialize};
pub fn day13() {
    env_logger::try_init().unwrap();
    let input = read_in_file("./day13/src/day13.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(13, result_1, result_2);
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Data {
    Integer(i32),
    List(Vec<Data>),
}
fn part1(input: &str) -> i32 {
    env_logger::try_init().unwrap_or(());
    let a: Vec<&str> = input.lines().collect();
    let mut res = 0;
    for (i, w) in a.windows(2).step_by(3).enumerate() {
        let v1: Vec<Data> = serde_json::from_str(w[0]).unwrap();
        let v2: Vec<Data> = serde_json::from_str(w[1]).unwrap();
        let (comp, _) = compare(&Data::List(v1), &Data::List(v2), false);
        if comp {
            info!("{} is correct", i + 1);
            res += i + 1;
        } else {
            info!("{} is not correct", i + 1);
        }
    }
    res as i32
}
fn compare(left: &Data, right: &Data, right_int: bool) -> (bool, bool) {
    debug!("Comparing {:#?} with {:#?}", left, right);
    match (left, right) {
        (Data::List(left_list), Data::List(right_list)) => {
            debug!("both are lists");
            for i in 0..left_list.len() {
                if i >= right_list.len() {
                    return (false, false);
                }
                let (comp, next) = compare(&left_list[i], &right_list[i], false);
                if next {
                    continue;
                }
                return (comp, false);
            }
            (true, false)
        }
        (Data::List(left_list), Data::Integer(_)) => {
            debug!("left is list, right is integer");
            if left_list.is_empty() {
                return (true, false);
            }
            compare(left, &Data::List(vec![right.clone()]), true)
        }
        (Data::Integer(_), Data::List(right_list)) => {
            debug!("left is integer, right is list");
            if right_list.is_empty() {
                return (false, false);
            }
            compare(&Data::List(vec![left.clone()]), right, true)
        }
        (Data::Integer(left_value), Data::Integer(right_value)) => {
            debug!("both are integers");
            if left_value == right_value {
                return (true, true);
            }
            (left_value <= right_value, false)
        }
    }
}
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(13, result)
    }
    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(0, result)
    }
}
