all: wasm-pack

wasm-pack:
	@echo "Building with wasm-pack..."
	wasm-pack build --release --target web --out-dir static

clean:
	cargo clean
	rm -rf static pkg

serve:
	python3 -m http.server 8080 --directory static
