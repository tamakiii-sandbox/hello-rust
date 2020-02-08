.PHONY: install run fmt clean

install:
	cargo install --path .

run:
	cargo run

fmt:
	cargo fmt

clean:
	rm -rf target

