.PHONY: run build ash clean

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

ash:
	docker run --rm -it -v $(PWD):$(WORK) -w $(WORK) $(NAME) $@

build:
	docker build -t $(NAME) --target $(ENVIRONMENT) .

clean:
	rm .env
	docker image rm $(NAME)

