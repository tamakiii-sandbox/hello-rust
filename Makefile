.PHONY: install run fmt clean

install:
	cargo install --path .

build:
	cargo build

run:
	cargo run

fmt:
	cargo fmt

clean:
	rm -rf target

