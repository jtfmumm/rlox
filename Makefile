
build:
	cargo build

test: build
	dart tool/bin/test.dart chap11_resolving --interpreter target/debug/lox-rust
