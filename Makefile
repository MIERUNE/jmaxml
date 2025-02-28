.PHONY: update test

update:
	uv run python -m jmx_codegen
#   cd jmaxml-go && go fmt ./... && go mod tidy
	cd ./jmaxml-rs && cargo fmt

test: update
	cd ./jmaxml-rs && cargo test
	cd ./jmaxml-json-types && npm run test
# 	cd jmaxml-go && go test ./... -coverprofile=coverage.txt -covermode=count && go tool cover -html coverage.txt
