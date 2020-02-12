.PHONY: install run fmt clean

%/new: Cargo.toml
	touch src/$(@D).rs
	echo "[[bin]]" >> $<
	echo "name = \"$(@D)\"" >> $<
	echo "path = \"src/$(@D).rs\"" >> $<

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
