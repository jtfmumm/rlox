
build:
	cargo build --release

test: build
	cd lox-test-suite && dart tool/bin/test.dart chap11_resolving --interpreter ../target/release/rlox
