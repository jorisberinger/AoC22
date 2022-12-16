use day01::day01;
use day02::day02;
use day03::day03;
use day04::day04;
use day05::day05;
use day06::day06;
use day07::day07;
use day08::day08;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    day01();
    day02();
    day03();
    day04();
    day05();
    day06();
    day07();
    day08();
    println!("Took {:?}", Instant::now() - start)
}
