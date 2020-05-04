RUSTUP = rustup

FMT_TOOLCHAIN = nightly-2020-01-15
FMT_CARGO = $(RUSTUP) run --install $(FMT_TOOLCHAIN) cargo --color always

TOOLCHAIN = $(shell cat rust-toolchain)
CARGO = $(RUSTUP) run --install $(TOOLCHAIN) cargo --color always

INSTALLED_COMPONENTS = $(shell $(RUSTUP) component list --installed --toolchain $(TOOLCHAIN))
FMT_INSTALLED_COMPONENTS = $(shell $(RUSTUP) component list --installed --toolchain $(FMT_TOOLCHAIN))

install_rust_fmt:
ifeq (,$(findstring $(FMT_TOOLCHAIN),$(INSTALLED_TOOLCHAINS)))
	$(RUSTUP) install $(FMT_TOOLCHAIN)
endif
ifeq (,$(findstring rustfmt,$(FMT_INSTALLED_COMPONENTS)))
	$(RUSTUP) component add rustfmt --toolchain $(FMT_TOOLCHAIN)
endif

install_rust:
ifeq (,$(findstring $(TOOLCHAIN),$(INSTALLED_TOOLCHAINS)))
	$(RUSTUP) install $(TOOLCHAIN)
endif

install_clippy: install_rust
ifeq (,$(findstring clippy,$(INSTALLED_COMPONENTS)))
	$(RUSTUP) component add clippy --toolchain $(TOOLCHAIN)
endif

install_wasm_pack:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

format: install_rust_fmt
	$(FMT_CARGO) fmt -- --files-with-diff

check_format: install_rust_fmt
	$(FMT_CARGO)  fmt -- --check

build: install_wasm_pack
	mkdir -p ./pkg/
	cp ./static/* ./pkg/
	wasm-pack build --debug --target web --out-name wasm

serve:
	python3 -m http.server --directory pkg

clippy: install_clippy
	$(CARGO) clippy --all-targets -- -D warnings

clean:
	$(CARGO) clean

test:
	$(CARGO) test
