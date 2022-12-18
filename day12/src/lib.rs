use std::{cmp, collections::HashMap, i32::MAX, mem};

use inputs::read_in_file;
use results::print_result;
pub fn day12() {
    let input = read_in_file("./day12/src/day12.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(12, result_1, result_2);
}
fn part1(input: &str) -> i32 {
    let (map, start, end) = read_in_map(input);
    let mut memory = HashMap::new();
    let result = step(&map, start, end, Vec::new(), &mut memory).unwrap();
    return result;
}
fn part2(input: &str) -> i32 {
    return 0;
}
fn step(
    map: &Vec<Vec<i32>>,
    position: (i32, i32),
    end: (i32, i32),
    visited: Vec<(i32, i32)>,
    memory: &mut HashMap<(i32, i32), Option<i32>>,
) -> Option<i32> {
    // println!("{:?}", memory);
    if position == end {
        // println!("{}, path: {:?}", visited.len(), visited);
        return Some(visited.len() as i32);
    }
    let mut value: Option<i32> = None;
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let (nx, ny) = (position.0 + dx, position.1 + dy);
        if nx >= 0 && ny >= 0 && nx < map.len() as i32 && ny < map[0].len() as i32
        // && !visited.contains(&(nx, ny))
        // && !memory.contains_key(&(nx, ny))
        {
            if let Some(score) = memory.get(&(nx, ny)) {
                if let Some(length) = score {
                    if *length <= visited.len() as i32 {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            let nv = map[(nx) as usize][(ny) as usize];
            let cv = map[position.0 as usize][position.1 as usize];
            if nv <= cv + 1 {
                let mut vi = visited.clone();
                vi.push(position);

                memory.insert(position, Some(visited.len() as i32));
                if let Some(score) = step(map, ((nx), (ny)), end, vi, memory) {
                    if let Some(best_score) = value {
                        value = Some(cmp::min(best_score, score));
                    } else {
                        value = Some(score)
                    }
                } else {
                    // println!("Burn {},{}", nx, ny);
                    if !memory.contains_key(&(nx, ny)) {
                        memory.insert((nx, ny), None);
                    }
                }
            }
        }
    }
    value
}
fn get_chars() -> HashMap<char, i32> {
    ('a'..='z')
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i as i32))
        .collect()
}
fn read_in_map(input: &str) -> (Vec<Vec<i32>>, (i32, i32), (i32, i32)) {
    let chars = get_chars();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut map = Vec::new();
    for (x, line) in input.lines().enumerate() {
        let mut values = Vec::new();
        for (y, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (x as i32, y as i32);
                values.push(0);
            } else if char == 'E' {
                end = (x as i32, y as i32);
                values.push(26);
            } else {
                let value = chars.get(&char).unwrap();
                values.push(value.to_owned());
            }
        }
        map.push(values);
    }
    (map, start, end)
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    #[test]
    fn part1_test() {
        // println!("{:?}", get_chars());
        let result = part1(INPUT);
        assert_eq!(31, result)
    }
    // #[test]
    // fn part2_test() {
    //     let result = part2(INPUT);
    //     assert_eq!(0, result)
    // }
    #[test]
    fn input_test() {
        let (result, start, end) = read_in_map(INPUT);
        assert_eq!(5, result.len());
        assert_eq!(8, result[0].len());
        assert_eq!((0, 0), start);
        assert_eq!((2, 5), end);
    }
}
