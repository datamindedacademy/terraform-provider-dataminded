# Host port to publish the API on. Override if 3000 is already taken:
#   make run PORT=3001
PORT ?= 3000

.PHONY: build run

build:
	docker build --platform linux/amd64 -t dm-api .


run: build
	docker run -p $(PORT):3000 -v $(shell pwd)/data:/data dm-api
