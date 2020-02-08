.PHONY: install run fmt clean

install:
	cargo install --path .

build:
	cargo $@

run:
	cargo $@

check:
	cargo $@

fmt:
	cargo $@

clean:
	rm -rf target

