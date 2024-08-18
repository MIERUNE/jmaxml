.PHONY: update test

update:
	python -m jmx_codegen
	rye format
	cd jmaxml-go && go fmt ./... && go mod tidy
	cd ./jmaxml-rs && cargo fmt

test:
	cd jmaxml-go && go test ./... -coverprofile=coverage.txt -covermode=count && go tool cover -html coverage.txt

    