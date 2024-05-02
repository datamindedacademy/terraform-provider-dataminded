

build:
	docker build --platform linux/amd64 -t dm-api .


run: build
	docker run -p 3000:3000 -v $(shell pwd)/data:/data dm-api