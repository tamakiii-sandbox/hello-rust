.PHONY: init build ash bash run run/% clean

NAME := tamakiii-sandbox/hello-rust
WORK := /app
ENVIRONMENT := $(shell grep '^ENVIRONMENT=' .env | sed -E 's/ENVIRONMENT=//')

init: \
	.env \
	build

.env:
	touch $@
	echo "ENVIRONMENT=production-pseudo" >> $@

run:
	docker run --rm -it -v $(PWD):$(WORK) -w $(WORK) $(NAME) make run

ash: run/ash
bash: run/bash

run/%:
	docker run --rm -it -v $(PWD):$(WORK) -w $(WORK) $(NAME) $(@F)

build:
	docker build -t $(NAME) --target $(ENVIRONMENT) .

clean:
	rm .env
	docker image rm $(NAME)

