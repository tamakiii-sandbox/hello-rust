
install:
	CARGO_HTTP_DEBUG=true CARGO_LOG=cargo::ops::registry=debug \
		cargo install -v cargo-edit cargo-inspect cargo-rls-install
