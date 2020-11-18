.PHONY=test
test: lint
	cargo test --all

.PHONY=run
run:
	cargo run

.PHONY=release
release: 
	cargo build --release

.PHONY=lint
lint:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

.PHONY=ci-release-%
ci-release-%:
	cargo build --release --target $*


.PHONY=local-cross-release
local-cross-release: local-release-aarch64-apple-darwin local-release-x86_64-apple-darwin local-release-x86_64-unknown-linux-gnu local-release-armv7-unknown-linux-gnueabihf

install-cross-toolchains:
	# https://github.com/messense/homebrew-macos-cross-toolchains
	brew tap messense/macos-cross-toolchains

local-release-x86_64-apple-darwin: export TARG = x86_64-apple-darwin
local-release-x86_64-apple-darwin:
	rustup target add $(TARG) && \
	\
	CC=cc \
	CXX=g++ \
	AR=ar \
	LINKER=gcc \
		cargo build --release --target $(TARG)

local-release-aarch64-apple-darwin: ## For releasing on Apple Silicon
	rustup target add aarch64-apple-darwin && \
		cargo build --release --target aarch64-apple-darwin

local-release-x86_64-unknown-linux-gnu: export TARG = x86_64-unknown-linux-gnu
local-release-x86_64-unknown-linux-gnu: install-cross-toolchains
	rustup target add $(TARG) && \
	brew install $(TARG) ; \
	\
	CC=$(TARG)-gcc \
	CXX=$(TARG)-g++ \
	AR=$(TARG)-ar \
	LINKER=$(TARG)-gcc \
		cross build --release --target $(TARG)

local-release-armv7-unknown-linux-gnueabihf: export TARG = armv7-unknown-linux-gnueabihf
local-release-armv7-unknown-linux-gnueabihf: install-cross-toolchains
	rustup target add $(TARG) && \
	\
	CC=$(TARG)-gcc \
	CXX=$(TARG)-g++ \
	AR=$(TARG)-ar \
	LINKER=$(TARG)-gcc \
		cargo build --release --target $(TARG)
