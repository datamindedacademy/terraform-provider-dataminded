SHELL := /bin/bash

.PHONY: default build api testacc install

default: build

# Compile the provider into ./bin, where the dev_overrides block in
# .terraformrc expects to find it.
build:
	go build -o ./bin/ .

# Run the Dataminded API on http://localhost:3000 (needs Docker).
# If that port is taken: make api PORT=3001
api:
	pushd api && make run

# Run acceptance tests
testacc:
	TF_ACC=1 go test ./... -v $(TESTARGS) -timeout 120m

# Install the provider on your PATH. Not needed for the exercises.
install:
	go install .
