.PHONY=test
test:
	cargo test

.PHONY=run
run:
	cargo run

.PHONY=release
release:
	cargo build --release
