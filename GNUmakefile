SHELL := /bin/bash
default: testacc

# Directory `go install` drops the provider binary in. Terraform's dev_overrides
# needs this as an absolute path, so we resolve it rather than hardcode it.
GOBIN_DIR := $(shell go env GOBIN)
ifeq ($(GOBIN_DIR),)
GOBIN_DIR := $(shell go env GOPATH)/bin
endif

.PHONY: default testacc api install dev clean

# Build the provider and point Terraform at the local binary.
dev: install .terraformrc
	@echo
	@echo "Provider installed in $(GOBIN_DIR)"
	@echo "Run this once per shell (the devcontainer sets it for you):"
	@echo
	@echo "    export TF_CLI_CONFIG_FILE=$(CURDIR)/.terraformrc"
	@echo

install:
	go install .

# Tells Terraform to use the binary you just built instead of downloading a
# released provider from a registry. Regenerate with `make clean dev` if you
# move the repo or change GOBIN.
.terraformrc:
	@printf 'provider_installation {\n  dev_overrides {\n    "hashicorp.com/dev/dataminded" = "%s"\n  }\n  direct {}\n}\n' '$(GOBIN_DIR)' > $@
	@echo "wrote $@ -> $(GOBIN_DIR)"

# Run the Data Minded API on http://localhost:3000 (needs Docker).
api:
	pushd api && make run

# Run acceptance tests
testacc:
	TF_ACC=1 go test ./... -v $(TESTARGS) -timeout 120m

clean:
	rm -f .terraformrc
