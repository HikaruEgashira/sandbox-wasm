build:
	cargo build --target wasm32-wasi
	cp target/wasm32-wasi/debug/wasi-demo.wasm .

run:
	wasmtime --dir=. target/wasm32-wasi/debug/wasi-demo.wasm test.txt tmp/output.txt