PHONY: release

release:
	cargo zigbuild --target x86_64-unknown-linux-gnu.2.17 --release
	strip ./target/x86_64-unknown-linux-gnu/release/bit