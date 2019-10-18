 
.PHONY: release, test

release:
	cargo build --release
	strip target/release/bit

build:
	cargo build

test:
	cargo test