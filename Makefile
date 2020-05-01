build:
	# wasm-pack overrides the git ignore file
	cp ./static/.gitignore ./static/.gitignore.bk
	wasm-pack build --debug --target web --out-name wasm --out-dir ./static
	mv ./static/.gitignore.bk ./static/.gitignore

serve:
	python3 -m http.server --directory static
