.PHONY: update test

update:
	python3 -m jmx_codegen
	black jmaxml-py
	go fmt ./...
	go mod tidy

test:
	go test ./... -coverprofile=coverage.txt -covermode=count

coverage: test
	go tool cover -html coverage.txt
    