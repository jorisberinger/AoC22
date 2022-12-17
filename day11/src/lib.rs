use inputs::read_in_file;
use regex::Regex;
use results::print_result;
use std::str::FromStr;
#[macro_use]
extern crate lazy_static;
pub fn day11() {
    let input = read_in_file("./day11/src/day11.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(11, result_1, result_2);
}
fn part1(input: &str) -> i32 {
    let mut monkeys = get_monkeys(input);
    for i in 0..20 {
        for j in 0..monkeys.len() {
            monkeys[j].clone().round(&mut monkeys);
        }
    }
    monkeys.sort_by_key(|x| x.inspections);
    return monkeys
        .iter()
        .rev()
        .take(2)
        .map(|x| x.inspections)
        .product();
}
fn part2(input: &str) -> i32 {
    return 0;
}
#[derive(Clone, Debug)]
struct Monkey {
    id: i32,
    starting_items: Vec<i32>,
    operator: String,
    value: i32,
    divisor: i32,
    true_monkey: i32,
    false_monkey: i32,
    inspections: i32,
}

impl Monkey {
    fn operation(&self, x: i32) -> i32 {
        let value = match self.value {
            0 => x,
            _ => self.value,
        };
        match self.operator.as_str() {
            "+" => x + value,
            "*" => x * value,
            &_ => panic!("unknown operator {}", self.operator.as_str()),
        }
    }
    fn round(&self, monkeys: &mut Vec<Monkey>) {
        for item in &self.starting_items {
            let inspected = self.operation(item.to_owned());
            monkeys[self.id as usize].inspections += 1;
            let seen = inspected / 3;
            let next_monkey = &self.test(seen);
            monkeys[*next_monkey as usize].starting_items.push(seen);
        }
        monkeys[self.id as usize].starting_items = Vec::new();
    }
    fn test(&self, x: i32) -> i32 {
        if x % self.divisor == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}
lazy_static! {
    static ref MONKEY_REG: regex::Regex = Regex::new(
        r"Monkey (\d+):\n\s*Starting items: ([\d,\s]+)\n\s*Operation: new = old ([*+]) ([\dold]+)\n\s*Test: divisible by (\d+)\n\s*If true: throw to monkey (\d+)\n\s*If false: throw to monkey (\d+)"
    )
    .unwrap();
}
fn get_monkeys(input: &str) -> Vec<Monkey> {
    let monkeys: Vec<Monkey> = MONKEY_REG
        .captures_iter(input)
        .map(|x| {
            let value = x.get(4).unwrap().as_str();
            let numv = if let Ok(v) = FromStr::from_str(value) {
                v
            } else {
                0
            };
            Monkey {
                id: FromStr::from_str(x.get(1).unwrap().as_str()).unwrap(),
                starting_items: x
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|x| FromStr::from_str(x).unwrap())
                    .collect(),
                operator: x.get(3).unwrap().as_str().to_string(),
                value: numv,
                divisor: FromStr::from_str(x.get(5).unwrap().as_str()).unwrap(),
                true_monkey: FromStr::from_str(x.get(6).unwrap().as_str()).unwrap(),
                false_monkey: FromStr::from_str(x.get(7).unwrap().as_str()).unwrap(),
                inspections: 0,
            }
        })
        .collect();
    monkeys
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(10605, result)
    }
    // #[test]
    // fn part2_test() {
    //     let result = part2(INPUT);
    //     assert_eq!(0, result)
    // }

    #[test]
    fn monkey_regex() {
        let monkeys = get_monkeys(INPUT);
        assert_eq!(4, monkeys.len());
        assert_eq!(0, monkeys[0].id);
        assert_eq!(1, monkeys[1].id);
        assert_eq!(vec![79, 98], monkeys[0].starting_items);
        assert_eq!(5, monkeys[3].operation(2));
        assert_eq!(4, monkeys[2].operation(2));
        assert_eq!(2, monkeys[0].test(23));
        assert_eq!(3, monkeys[0].test(24));
    }
}
