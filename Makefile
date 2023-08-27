.PHONY: clean build build-test release test lint fmt-check docs
.DEFAULT_GOAL := build

clean:
	cargo clean

build:
	cargo build

build-test:
	cargo build --tests

release:
	cargo build --release

test:
	RUST_LOG=warn cargo test

lint:
	cargo clippy -- -D warnings

fmt-check:
	cargo fmt -- --check

docs:
	cargo doc --no-deps
