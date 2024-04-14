default: testacc

# Run acceptance tests
.PHONY: testacc api
testacc:
	TF_ACC=1 go test ./... -v $(TESTARGS) -timeout 120m

api:
	pushd api && RUST_LOG=debug cargo run