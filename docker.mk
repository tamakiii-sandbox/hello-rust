.PHONY: init build ash bash run run/% make/% clean
.DEFAULT_GOAL := help

DIR := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))
NAME := tamakiii-sandbox/hello-rust
WORK := /app
ENVIRONMENT := $(shell grep '^ENVIRONMENT=' .env | sed -E 's/ENVIRONMENT=//')
VOLUMES := $(DIR):$(WORK) ~/.gitignore:/root/.gitignore ~/.config/git/ignore:/root/.config/git/ignore
PORTS := 8800:8000

CMD := bash
VSC_NAME := vsc-hello-rust-

help:
	cat $(lastword $(MAKEFILE_LIST))

init: \
	.env \
	build

.env:
	touch $@
	echo "ENVIRONMENT=production-pseudo" >> $@

build:
	docker build -t $(NAME) --target $(ENVIRONMENT) .

run:
	docker run --rm -it $(foreach p,$(PORTS),-p $p) $(foreach v,$(VOLUMES),-v $v) -w $(WORK) $(NAME) make run

ash: run/ash
bash: run/bash

run/%:
	docker run --rm -it $(foreach p,$(PORTS),-p $p) $(foreach v,$(VOLUMES),-v $v) -w $(WORK) $(NAME) $(@F)

make/%:
	docker run --rm -it $(foreach p,$(PORTS),-p $p) $(foreach v,$(VOLUMES),-v $v) -w $(WORK) $(NAME) make $(@F)

vsc:
	docker exec -it -w /workspaces/$(shell basename $(realpath $(dir $(lastword $(MAKEFILE_LIST))))) $$(docker container ls | grep $(VSC_NAME) | awk '{ print $$1 }') $(CMD)

clean:
	rm .env
	docker image rm $(NAME)

