.PHONY: install run fmt clean

BIN :=

new:
	cargo $@ --bin $(BIN)

build:
	cargo $@ --bin $(BIN)

run:
	cargo $@ --bin $(BIN)

check:
	cargo $@ --bin $(BIN)

fmt:
	cargo $@ --bin $(BIN)

clean:
	cargo $@ --bin $(BIN)

