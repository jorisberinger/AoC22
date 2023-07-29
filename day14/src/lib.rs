use std::collections::HashMap;

use inputs::read_in_file;
use results::print_result;
struct Cave {
    rocks: HashMap<(i32, i32), bool>,
    floor: Option<i32>,
}
impl Cave {
    fn new(floor: Option<i32>) -> Self {
        Self {
            rocks: HashMap::new(),
            floor,
        }
    }
    fn add_rock(&mut self, x: i32, y: i32) {
        self.rocks.insert((x, y), true);
    }
    fn add_rocks(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        for x in x1.min(x2)..=x1.max(x2) {
            for y in y1.min(y2)..=y1.max(y2) {
                self.add_rock(x, y);
            }
        }
    }
    fn get_depth(&self) -> i32 {
        self.rocks.keys().map(|(_, y)| y).max().unwrap().clone()
    }
    fn drop_sand(&mut self, position: (i32, i32)) -> Result<(), ()> {
        if self.rocks.contains_key(&position) {
            return Err(());
        }
        match self.floor {
            Some(floor) => {
                if position.1 == floor {
                    self.add_rock(position.0, position.1);
                    return Ok(());
                }
            }
            None => {
                if position.1 > self.get_depth() {
                    return Err(());
                }
            }
        }
        let below = (position.0, position.1 + 1);
        let below_left = (position.0 - 1, position.1 + 1);
        let below_right = (position.0 + 1, position.1 + 1);
        if !self.rocks.contains_key(&below) {
            return self.drop_sand(below);
        }
        if !self.rocks.contains_key(&below_left) {
            return self.drop_sand(below_left);
        }
        if !self.rocks.contains_key(&below_right) {
            return self.drop_sand(below_right);
        }
        self.add_rock(position.0, position.1);
        return Ok(());
    }
    fn add_floor(&mut self) {
        self.floor = Some(self.get_depth() + 1)
    }
}
impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let mut cave = Self::new(None);
        for line in input.lines() {
            let parts: Vec<&str> = line.split(" -> ").collect();
            // go through the the parts in pairs of two

            for pair in parts.windows(2) {
                let mut first = pair[0].split(",");
                let x1 = first.next().unwrap().parse::<i32>().unwrap();
                let y1 = first.next().unwrap().parse::<i32>().unwrap();
                let mut second = pair[1].split(",");
                let x2 = second.next().unwrap().parse::<i32>().unwrap();
                let y2 = second.next().unwrap().parse::<i32>().unwrap();

                cave.add_rocks(x1, y1, x2, y2);
            }
        }
        cave
    }
}
pub fn day14() {
    let input = read_in_file("./day14/src/day14.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(14, result_1, result_2);
}
fn part1(input: &str) -> i32 {
    let mut cave = Cave::from(input);
    let before = cave.rocks.len();
    loop {
        match cave.drop_sand((500, 0)) {
            Ok(_) => {}
            Err(_) => break,
        }
    }
    return cave.rocks.len() as i32 - before as i32;
}
fn part2(input: &str) -> i32 {
    let mut cave = Cave::from(input);
    cave.add_floor();
    let before = cave.rocks.len();
    loop {
        match cave.drop_sand((500, 0)) {
            Ok(_) => {}
            Err(_) => break,
        }
    }
    return cave.rocks.len() as i32 - before as i32;
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(24, result)
    }
    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(93, result)
    }
    #[test]
    fn create_cave() {
        let mut cave = Cave::new(None);
        cave.add_rock(0, 0);
        cave.add_rocks(0, 3, 0, 4);

        assert_eq!(3, cave.rocks.len())
    }
    #[test]
    fn create_cave_from_input() {
        let cave = Cave::from(INPUT);
        assert_eq!(20, cave.rocks.len())
    }
}
