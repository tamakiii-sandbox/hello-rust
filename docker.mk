.PHONY: init build ash bash run run/% make/% clean
.DEFAULT_GOAL := help

NAME := tamakiii-sandbox/hello-rust
WORK := /app
ENVIRONMENT := $(shell grep '^ENVIRONMENT=' .env | sed -E 's/ENVIRONMENT=//')
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
	docker run --rm -it -v $(PWD):$(WORK) -w $(WORK) $(NAME) make run

ash: run/ash
bash: run/bash

run/%:
	docker run --rm -it -v $(PWD):$(WORK) -w $(WORK) $(NAME) $(@F)

make/%:
	docker run --rm -it -v $(PWD):$(WORK) -w $(WORK) $(NAME) make $(@F)

vsc:
	docker exec -it $$(docker container ls | grep $(VSC_NAME) | awk '{ print $$1 }') $(CMD)

clean:
	rm .env
	docker image rm $(NAME)

