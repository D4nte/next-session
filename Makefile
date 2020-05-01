build:
	wasm-pack build --debug --target web --out-name wasm --out-dir ./static

serve:
	python3 -m http.server --directory static
