run:
	cargo-watch -x check -x "nextest run" -x "run --release"
test:
	cargo-watch -x check -x "nextest run"
