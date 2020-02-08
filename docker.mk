.PHONY: run build clean

NAME := tamakiii-sandbox/hello-rust
WORK := /app

run:
	docker run --rm -it -v $(PWD):$(WORK) -w $(WORK) $(NAME) make run

build:
	docker build -t $(NAME) .

clean:
	docker image rm $(NAME)

