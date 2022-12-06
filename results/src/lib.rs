pub fn print_result<T1: std::fmt::Display, T2: std::fmt::Display>(day: u8, part1: T1, part2: T2) {
    println!("{}", format_result(day, part1, part2))
}

fn format_result<T1: std::fmt::Display, T2: std::fmt::Display>(
    day: u8,
    part1: T1,
    part2: T2,
) -> String {
    return String::from(format!(
        "Day {:0>2}:\tPart 1: {}\tPart 2: {}",
        day, part1, part2
    ));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_test() {
        let string = format_result(1, 123, 456);
        assert_eq!("Day 01:\tPart 1: 123\tPart 2: 456", string);
    }
}
