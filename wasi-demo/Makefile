install:
	curl https://wasmtime.dev/install.sh -sSf | bash

build:
	cargo build --target wasm32-wasi
	cp target/wasm32-wasi/debug/wasi-demo.wasm .

run:
	wasmtime --dir=. wasi-demo.wasm .