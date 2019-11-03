 
.PHONY: release, test, dev

release:
	cargo build --release
	strip target/release/bit

build:
	cargo build

dev:
	. ./ENV.sh; \
	cargo run --bin website;

test:
	cargo test