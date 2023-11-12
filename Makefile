.PHONY: all run test format lint

all: run

run:
    cargo run

test:
    cargo test

format:
    cargo fmt

lint:
    cargo clippy


clean:
    cargo clean

check:
    cargo check


