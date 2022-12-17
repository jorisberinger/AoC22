run:
	cargo-watch -x check -x "nextest run" -x "run --release"
test:
	cargo-watch -x check -x "nextest run"
day: delete-day
	cargo new $(day) --lib
	cargo member include $(day)
	cargo add $(day) -p app --path './$(day)'
	cargo add results -p $(day) --path './results'
	cargo add inputs -p $(day) --path './inputs'
	touch ./$(day)/src/$(day).txt
	rm ./$(day)/src/lib.rs
	echo 'use inputs::read_in_file; \
				use results::print_result; \
				pub fn $(day)() { \
    			let input = read_in_file("./$(day)/src/$(day).txt"); \
    			let result_1 = part1(&input); \
    			let result_2 = part2(&input); \
					print_result($(day:day=''), result_1, result_2); \
				} \
				fn part1(input: &str) -> i32 { \
					todo!(); \
				} \
				fn part2(input: &str) -> i32 { \
					todo!(); \
				} \
				#[cfg(test)] \
				mod tests { \
				use super::*; \
				const INPUT: &str = "replace"; \
				#[test] \
				fn part1_test() { \
					let result = part1(INPUT); \
								assert_eq!(0, result) \
					} \
					#[test] \
					fn part2_test() { \
									let result = part2(INPUT); \
									assert_eq!(0, result) \
								} \
				} \
	' >> ./$(day)/src/lib.rs
	cargo fmt -- ./$(day)/src/lib.rs
	echo 'Call $(day)() in app/src/main.rs'



delete-day:
	# cargo member exclude $(day)
	rm -r -f $(day)
