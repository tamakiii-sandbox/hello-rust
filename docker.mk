.PHONY: init build ash bash run run/% make/% clean

NAME := tamakiii-sandbox/hello-rust
WORK := /app
ENVIRONMENT := $(shell grep '^ENVIRONMENT=' .env | sed -E 's/ENVIRONMENT=//')

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

clean:
	rm .env
	docker image rm $(NAME)

