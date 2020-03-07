build:
	@RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/noise.wasm .

run: build
	wasmer noise.wasm