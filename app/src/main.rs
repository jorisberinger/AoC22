use day01::day01;
use day02::day02;
use day03::day03;
use day04::day04;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    day01();
    day02();
    day03();
    day04();
    println!("Took {:?}", Instant::now() - start)
}
