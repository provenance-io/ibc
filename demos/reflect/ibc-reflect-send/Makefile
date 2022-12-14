UNAME_M := $(shell uname -m)

.PHONY: all
all: fmt build lint schema optimize

.PHONY: fmt
fmt:
	@cargo fmt --all -- --check

.PHONY: build
build:
	@cargo wasm

.PHONY: test
test:
	@RUST_BACKTRACE=1 cargo unit-test

.PHONY: lint
lint:
	@cargo clippy -- -D warnings

.PHONY: schema
schema:
	@cargo schema

.PHONY: optimize
optimize:
ifeq ($(UNAME_M),arm64)
	@docker run --rm -v $(CURDIR):/code \
		--mount type=volume,source="ibc-reflect-send_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer-arm64:0.12.8
else
	@docker run --rm -v $(CURDIR):/code \
		--mount type=volume,source="ibc-reflect-send_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.12.8
endif
