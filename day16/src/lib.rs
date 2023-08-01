use core::fmt;
use inputs::read_in_file;
// use rayon::prelude::*;
use regex::Regex;
use results::print_result;
use std::collections::HashMap;
pub fn day16() {
    let input = read_in_file("./day16/src/day16.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input);
    print_result(16, result_1, result_2);
}
fn part1(input: &str) -> i32 {
    let cave_map = CaveMap::from(input);
    println!("{}", cave_map);
    let mut open_valves = Vec::new();
    let result = cave_map.get_path("AA", 20, &mut open_valves, &Vec::new());
    return result;
}
fn part2(input: &str) -> i32 {
    return 0;
}

#[derive(Debug)]
struct Cave {
    valve_id: String,
    flow_rate: i32,
    tunnels: Vec<String>,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cave {} has flow rate {} and tunnels to {:?}",
            self.valve_id, self.flow_rate, self.tunnels
        )
    }
}
#[derive(Debug)]
pub struct CaveMap {
    caves: HashMap<String, Cave>,
}

impl CaveMap {
    pub fn get_path<'a>(
        &'a self,
        start: &'a str,
        minutes: i32,
        open_valves: &'a mut Vec<&'a str>,
        visited_valves: &Vec<&str>,
    ) -> i32 {
        // println!(
        //     "{} Cave {}, minutes {} open_valves {:?}",
        //     " ".repeat(5 - minutes as usize),
        //     start,
        //     minutes,
        //     open_valves
        // );
        if minutes <= 0 {
            return 0;
        }

        let cave = self.caves.get(start).expect("Cave not found");

        let flow = cave.flow_rate * (minutes - 1);

        // let mut with_open_valve = open_valves.clone();
        // with_open_valve.push(start);
        open_valves.push(start);

        let mut with_visited_valve = visited_valves.clone();
        with_visited_valve.push(start);

        let option_open = match open_valves.contains(&start) || cave.flow_rate == 0 {
            false => flow + self.get_path(start, minutes - 1, open_valves, &with_visited_valve),
            true => 0,
        };
        let option_closed = cave
            .tunnels
            .iter()
            // .par_iter()
            // .filter(|tunnel| !visited_valves.contains(&tunnel.as_str()))
            .map(|tunnel| self.get_path(&tunnel, minutes - 1, open_valves, &with_visited_valve))
            .max()
            .unwrap_or(0);

        // println!(
        //     "Cave {}, minutes {},  option_open {}, option_closed {}",
        //     start, minutes, option_open, option_closed
        // );
        return option_open.max(option_closed);
    }
}
impl fmt::Display for CaveMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for cave in self.caves.values() {
            result.push_str(&format!("{}\n", cave));
        }
        write!(f, "{}", result)
    }
}
impl From<&str> for CaveMap {
    fn from(value: &str) -> Self {
        let cave_info_pattern = Regex::new(r"Valve (?P<valve_id>\w+) has flow rate=(?P<flow_rate>\d+); tunnel[s]? lead[s]? to valve[s]? (?P<tunnels>[\w, ]+)").unwrap();

        let mut cave_map: HashMap<String, Cave> = HashMap::new();

        for caps in cave_info_pattern.captures_iter(value) {
            let valve_id = caps["valve_id"].to_string();
            let flow_rate = caps["flow_rate"].parse().unwrap();
            let tunnels = caps["tunnels"]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            cave_map.insert(
                valve_id.clone(),
                Cave {
                    valve_id,
                    flow_rate,
                    tunnels,
                },
            );
        }
        Self { caves: cave_map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(1651, result)
    }
    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(0, result)
    }
    #[test]
    fn read_in_caves() {
        let cave_map = CaveMap::from(INPUT);
        println!("{}", cave_map);
        assert_eq!(10, cave_map.caves.len());

        assert_eq!(0, cave_map.caves["AA"].flow_rate);
        assert_eq!(13, cave_map.caves["BB"].flow_rate);
        assert_eq!(2, cave_map.caves["CC"].flow_rate);
        assert_eq!(20, cave_map.caves["DD"].flow_rate);
        assert_eq!(3, cave_map.caves["EE"].flow_rate);
        assert_eq!(0, cave_map.caves["FF"].flow_rate);
        assert_eq!(0, cave_map.caves["GG"].flow_rate);
        assert_eq!(22, cave_map.caves["HH"].flow_rate);
        assert_eq!(0, cave_map.caves["II"].flow_rate);
        assert_eq!(21, cave_map.caves["JJ"].flow_rate);
    }
}
