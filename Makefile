RUSTUP = rustup

FMT_TOOLCHAIN = nightly-2020-01-15
FMT_CARGO = $(RUSTUP) run --install $(FMT_TOOLCHAIN) cargo --color always

TOOLCHAIN = $(shell cat rust-toolchain)
CARGO = $(RUSTUP) run --install $(TOOLCHAIN) cargo --color always

INSTALLED_COMPONENTS = $(shell $(RUSTUP) component list --installed --toolchain $(TOOLCHAIN))

install_rust_fmt:
ifeq (,$(findstring $(FMT_TOOLCHAIN),$(INSTALLED_TOOLCHAINS)))
	$(RUSTUP) install $(FMT_TOOLCHAIN)
endif

install_rust:
ifeq (,$(findstring $(TOOLCHAIN),$(INSTALLED_TOOLCHAINS)))
	$(RUSTUP) install $(TOOLCHAIN)
endif

install_clippy: install_rust
ifeq (,$(findstring clippy,$(INSTALLED_COMPONENTS)))
	$(RUSTUP) component add clippy --toolchain $(TOOLCHAIN)
endif

format:
	$(FMT_CARGO) fmt -- --files-with-diff

check_format:
	$(FMT_CARGO)  fmt -- --check

build:
	# wasm-pack overrides the git ignore file
	cp ./static/.gitignore ./static/.gitignore.bk
	wasm-pack build --debug --target web --out-name wasm --out-dir ./static
	mv ./static/.gitignore.bk ./static/.gitignore

serve:
	python3 -m http.server --directory static

clippy: install_clippy
	$(CARGO) clippy --all-targets -- -D warnings

clean:
	$(CARGO) clean
