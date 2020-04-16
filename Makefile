 
.PHONY: release, test, dev

release:
	cargo build --release --bin api
	strip target/release/api

build:
	cargo build

# DEV target
# First load ENV variables,
# then starts server
dev:
	. ./ENV.sh; \
	cargo run --bin api;

test:
	cargo test