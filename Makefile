RUSTUP = rustup

FMT_TOOLCHAIN = nightly-2020-01-15

format:
	$(RUSTUP) run --install $(FMT_TOOLCHAIN) cargo --color always fmt -- --files-with-diff

build:
	# wasm-pack overrides the git ignore file
	cp ./static/.gitignore ./static/.gitignore.bk
	wasm-pack build --debug --target web --out-name wasm --out-dir ./static
	mv ./static/.gitignore.bk ./static/.gitignore

serve:
	python3 -m http.server --directory static
