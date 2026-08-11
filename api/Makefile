# Host port to publish the API on. Override if 3000 is already taken:
#   make run PORT=3001
PORT ?= 3000

.PHONY: build run up down logs

build:
	docker build --platform linux/amd64 -t dm-api .

# Run in the foreground. Ctrl-C stops it.
run: build
	docker run -p $(PORT):3000 -v $(shell pwd)/data:/data dm-api

# Run detached, as the devcontainer does on start. Recreates the container so
# repeated invocations are idempotent.
up: build
	-docker rm -f dm-api 2>/dev/null
	docker run -d --name dm-api -p $(PORT):3000 -v $(shell pwd)/data:/data dm-api

down:
	-docker rm -f dm-api 2>/dev/null

logs:
	docker logs -f dm-api
