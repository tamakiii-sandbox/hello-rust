.PHONY: run build clean

NAME := tamakiii-sandbox/hello-rust
WD := /app

run:
	docker run --rm -it -v $(PWD):$(WD) -w $(WD) $(NAME) make run

build:
	docker build -t $(NAME) .

clean:
	docker image rm $(NAME)

