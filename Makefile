.PHONY: update test

update:
	python -m jmx_codegen
	rye format
	go fmt ./...
	go mod tidy
	cd ./jmaxml-rs && cargo fmt

test:
	go test ./... -coverprofile=coverage.txt -covermode=count

coverage: test
	go tool cover -html coverage.txt
    