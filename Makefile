CPY_FILES = index.html wasm_game_engine_bg.wasm wasm_game_engine.js

all: wasm-pack

wasm-pack:
	@echo "Building with wasm-pack..."
	wasm-pack build --release --target web --out-dir static

clean:
	cargo clean
	rm -rf static pkg

serve:
	python3 -m http.server 8080 --directory static

copy:
	@echo "Copying files..."
	@mkdir -p docs
	@for file in $(CPY_FILES); do \
		cp "./static/$$file" "./docs/"; \
	done
	@echo "Done!"
