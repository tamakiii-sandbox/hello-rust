.PHONY: install run fmt clean

%/new:
	cargo $(@F) --bin $(@D)

%/build:
	cargo $(@F) --bin $(@D)

%/run:
	cargo $(@F) --bin $(@D)

%/check:
	cargo $(@F) --bin $(@D)

%/fmt:
	cargo $(@F) --bin $(@D)

%/clean:
	cargo $(@F) --bin $(@D)
