 
.PHONY: release, test, dev

release:
	cargo build --release
	strip target/release/website

build:
	cargo build

# DEV target
# First load ENV variables,
# then starts server
dev:
	. ./ENV.sh; \
	cargo run --bin website;

test:
	cargo test