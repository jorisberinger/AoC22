use regex::Regex;
use std::collections::HashMap;

use inputs::read_in_file;
use results::print_result;
pub fn day15() {
    let input = read_in_file("./day15/src/day15.txt");
    let result_1 = part1(&input);
    let result_2 = part2(&input, 4_000_000);
    print_result(15, result_1, result_2);
}
fn part1(input: &str) -> i32 {
    let ground = Ground::from(input);
    let occupied = ground.row_occupied(2000000);
    return occupied as i32;
}
fn part2(input: &str, limit: i32) -> i32 {
    let mut ground = Ground::from(input);
    ground.add_limit(limit);
    let distress = ground.get_distress();
    return distress.0 * 4_000_000 + distress.1;
}

struct Ground {
    sensors: Vec<(i32, i32, i32)>,
    beacons: Vec<(i32, i32)>,
    grid: Vec<(i32, i32, i32)>,
    limit: Option<i32>,
}

impl Ground {
    fn new() -> Self {
        Ground {
            grid: Vec::new(),
            sensors: Vec::new(),
            beacons: Vec::new(),
            limit: None,
        }
    }
    fn row_occupied(&self, row: i32) -> usize {
        let min = self.sensors.iter().map(|s| s.0 - s.2).min().unwrap();
        let max = self.sensors.iter().map(|s| s.0 + s.2).max().unwrap();

        let mut occupied = 0;
        'y: for i in min..=max {
            let position = (i, row);
            for beacon in &self.beacons {
                if beacon == &position {
                    // println!("beacon at {:?}", position);
                    continue 'y;
                }
            }

            for sensor in &self.sensors {
                if sensor.0 == position.0 && sensor.1 == position.1 {
                    // println!("sensor at {:?}", position);
                    continue 'y;
                }
            }
            for sensor in &self.sensors {
                let distance = (sensor.0 - position.0).abs() + (sensor.1 - position.1).abs();
                if distance <= sensor.2 {
                    // println!("occupied at {:?}", position);
                    occupied += 1;
                    continue 'y;
                }
            }
        }
        occupied
    }

    fn add_limit(&mut self, limit: i32) {
        self.limit = Some(limit);
    }

    fn get_distress(&self) -> (i32, i32) {
        for y in 0..self.limit.unwrap() {
            let mut ranges: Vec<&(i32, i32, i32)> = self.grid.iter().filter(|g| g.0 == y).collect();
            // println!("");
            // print!("{:2o} ", y);
            // println!("ranges {:?}", ranges);
            println!("{}", y);
            // sort ranges by start
            ranges.sort_by(|a, b| a.1.cmp(&b.1));
            let first = ranges.first().unwrap();
            if first.1 > 0 {
                return (0, y);
            }
            let mut end = first.2;
            for range in ranges {
                if range.1 > end + 1 {
                    return (end + 1, y);
                }
                if range.2 > end {
                    end = range.2;
                }
            }
            if end < self.limit.unwrap() {
                return (end, y);
            }
        }
        (0, 0)
    }
}

impl From<&str> for Ground {
    fn from(value: &str) -> Self {
        let mut ground = Ground::new();
        let sensor_re =
            Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+)").unwrap();
        let beacon_re =
            Regex::new(r"closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)")
                .unwrap();
        for line in value.lines() {
            let sensor_caps = sensor_re.captures(line).unwrap();
            let sensor_x = sensor_caps["sensor_x"].parse::<i32>().unwrap();
            let sensor_y = sensor_caps["sensor_y"].parse::<i32>().unwrap();
            let beacon_caps = beacon_re.captures(line).unwrap();
            let beacon_x = beacon_caps["beacon_x"].parse::<i32>().unwrap();
            let beacon_y = beacon_caps["beacon_y"].parse::<i32>().unwrap();
            let manhatten_distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
            ground
                .sensors
                .push((sensor_x, sensor_y, manhatten_distance));
            ground.beacons.push((beacon_x, beacon_y));

            for i in 0..=manhatten_distance {
                // println!(
                //     "({}, {}) {} {} {}-{}",
                //     sensor_x,
                //     sensor_y,
                //     manhatten_distance,
                //     sensor_y - i,
                //     sensor_x - manhatten_distance + i,
                //     sensor_x + manhatten_distance - i
                // );
                ground.grid.push((
                    sensor_y - i,
                    sensor_x - manhatten_distance + i,
                    sensor_x + manhatten_distance - i,
                ));
                ground.grid.push((
                    sensor_y + i,
                    sensor_x - manhatten_distance + i,
                    sensor_x + manhatten_distance - i,
                ));
            }
        }
        ground
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
    #[test]
    fn part1_test() {
        let ground = Ground::from(INPUT);
        let result = ground.row_occupied(10);
        assert_eq!(26, result)
    }
    #[test]
    fn part2_test() {
        let result = part2(INPUT, 20);
        assert_eq!(56000011, result)
    }
}
