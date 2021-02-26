.PHONY: build clean standalone

build:
	mkdir -p wasm
	rm -rf target/wasm32-unknown-emscripten/release/deps/*.wasm
	rm -rf target/wasm32-unknown-emscripten/release/rustynes.js
	cargo rustc --release --bin rustynes \
	--target=wasm32-unknown-emscripten -- \
    -C opt