
build:
	./build.sh codelta_backend


release:
	docker build -t codelta_backend .
	mkdir -p $(shell pwd)/release-artifacts
	docker run --rm -v $(shell pwd)/release-artifacts:/target/wasm32-unknown-unknown/release codelta_backend
	shasum -a 256 $(shell pwd)/release-artifacts/codelta_backend.wasm  | cut -d ' ' -f 1
