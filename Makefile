 
.PHONY: release, test, run, build

release:
	cargo update
	cargo build --release
	strip target/release/bit

build:
	cargo update
	cargo build

# DEV target
# First load ENV variables,
# then starts server
run:
	cargo update
	cargo run

test:
	cargo update
	cargo test