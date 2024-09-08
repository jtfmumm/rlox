
.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	rm -rf target

.PHONY: test
test: build
	cd lox-test-suite && dart tool/bin/test.dart chap11_resolving --interpreter ../target/release/rlox

.PHONY: lint
lint:
	cargo check
	cargo clippy

.PHONY: lint-fix
lint-fix: # fix lints where possible
	cargo fix
	cargo clippy --fix

.PHONY: format-check
format: # check formatting
	cargo fmt --check

.PHONY: format-fix
format-fix: # fix formatting
	cargo fmt
