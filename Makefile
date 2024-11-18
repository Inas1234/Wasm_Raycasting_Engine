CPY_FILES = index.html wasm_game_engine_bg.wasm wasm_game_engine.js
PNG_FILES = $(wildcard static/*.png)

all: wasm-pack

wasm-pack:
	@echo "Building with wasm-pack..."
	wasm-pack build --release --target web --out-dir static

clean:
	cargo clean
	rm -rf static pkg

serve:
	python3 -m http.server 8000 --directory static

copy:
	@echo "Copying files..."
	@mkdir -p docs
	@for file in $(CPY_FILES); do \
		cp "./static/$$file" "./docs/"; \
	done
	@for png in $(PNG_FILES); do \
		cp "$$png" "./docs/"; \
	done
	@echo "Done!"
