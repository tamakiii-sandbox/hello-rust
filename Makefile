.PHONY: install clean

install:
	cargo install --path .

run:
	cargo run

clean:
	rm -rf target

