use inputs::read_in_file;
use regex::Captures;
use regex::Regex;
use results::print_result;
pub fn day05() {
    let input = read_in_file("./day05/src/day05.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(5, result_1, result_2);
}
fn part1(input: &str) -> String {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 10];
    for line in input.lines() {
        for i in (1..line.len()).step_by(4) {
            let char = line.chars().nth(i).unwrap();
            if char.is_whitespace() || !char.is_alphabetic() {
                continue;
            };
            if let Some(_) = stacks.get(i / 4) {
                stacks[i / 4].insert(0, char)
            } else {
                stacks[i / 4] = vec![char]
            }
        }
        if line.is_empty() {
            break;
        }
    }
    let re = Regex::new(r"move (\d+).+(\d+).+(\d+)").unwrap();
    for line in input.lines() {
        if let Some(captures) = re.captures(line) {
            let s1 = get_match(&captures, 1);
            let s2 = get_match(&captures, 2);
            let s3 = get_match(&captures, 3);
            for _ in 0..s1 {
                if let Some(value) = stacks[s2 as usize - 1].pop() {
                    stacks[s3 as usize - 1].push(value)
                }
            }
        }
    }
    let mut result = String::new();
    for s in stacks {
        if let Some(char) = s.last() {
            result.push_str(char.to_string().as_str())
        }
    }

    return result;
}
fn get_match(captures: &Captures, i: usize) -> i32 {
    captures
        .get(i)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .unwrap()
}
fn part2(input: &str) -> String {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 10];
    for line in input.lines() {
        for i in (1..line.len()).step_by(4) {
            let char = line.chars().nth(i).unwrap();
            if char.is_whitespace() || !char.is_alphabetic() {
                continue;
            };
            if let Some(_) = stacks.get(i / 4) {
                stacks[i / 4].insert(0, char)
            } else {
                stacks[i / 4] = vec![char]
            }
        }
        if line.is_empty() {
            break;
        }
    }
    let re = Regex::new(r"move (\d+).+(\d+).+(\d+)").unwrap();
    for line in input.lines() {
        if let Some(captures) = re.captures(line) {
            let s1 = get_match(&captures, 1);
            let s2 = get_match(&captures, 2);
            let s3 = get_match(&captures, 3);
            let mut temp = Vec::new();
            for _ in 0..s1 {
                if let Some(value) = stacks[s2 as usize - 1].pop() {
                    temp.push(value)
                }
            }
            for _ in 0..s1 {
                if let Some(value) = temp.pop() {
                    stacks[s3 as usize - 1].push(value);
                }
            }
        }
    }
    let mut result = String::new();
    for s in stacks {
        if let Some(char) = s.last() {
            result.push_str(char.to_string().as_str())
        }
    }

    return result;
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!("CMZ", result)
    }
    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!("MCD", result)
    }
}
