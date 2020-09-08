WASM         := target/wasm32-wasi/release/hello-wasi.wasm
WAPM_PACKAGE := wapm_packages/.bin/hello-wasi

$(WASM): Cargo.toml src/main.rs
	cargo +nightly build --release --target=wasm32-wasi

$(WAPM_PACKAGE): $(WASM)
	wapm install

run: $(WAPM_PACKAGE)
	@$(WAPM_PACKAGE)

.PHONY: run
