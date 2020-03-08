.PHONY: install clean

PROJECT := project/...

install: \
	$(PROJECT) \
	src \
	Cargo.toml \
	Cargo.lock

src:
	ln -s $(PROJECT)/$@ .

Cargo.toml:
	ln -s $(PROJECT)/$@ .

Cargo.lock:
	ln -s $(PROJECT)/$@ .

clean:
	rm -rf src Cargo.toml Cargo.lock

require:
ifeq ($(PROJECT),)
	$(error "You must specify PROJECT #=> like $$ make PROJECT=project/hello")
endif