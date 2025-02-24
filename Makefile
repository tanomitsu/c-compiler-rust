docker-build:
	docker build -t c-compiler-test .

test: docker-build
	docker run --rm -it --platform=linux/amd64 c-compiler-test ./test.sh

clean:
	cargo clean

.PHONY: test clean