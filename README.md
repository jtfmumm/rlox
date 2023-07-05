# rlox

A Rust implementation of an interpreter for the Lox language.

## Overview

Lox is a language designed by [Bob Nystrom](https://journal.stuffwithstuff.com/) for his entertaining book [Crafting Interpreters](https://craftinginterpreters.com/).

This Rust-based Lox interpreter is currently a simple tree-walking interpreter. If for some strange reason you work at a Lox shop, don't use this interpreter in production!

## Implemented

Tree-walking interpreter

- [x] scanning/lexing
- [x] parsing
- [x] evaluating
- [x] statements
- [x] control flow
- [x] functions
- [x] scope resolution
- [ ] classes
- [ ] inheritance

Future: implement the bytecode-based version.

## Usage

Clone and change to `rlox` directory:
```
git clone https://github.com/jtfmumm/rlox
cd rlox
```

Build:
```
cargo build --release
```

Enter the repl:
```
target/release/rlox
```

Run a Lox file:
```
target/release/rlox examples/hello-world.lox
```

## Tests

I've included Bob Nystrom's Lox interpreter test suite.

To run, you need to have [Dart](https://dart.dev) v2.12. On MacOS you can install with:

```
brew install dart@2.12
```

Then install deps with:

```
cd lox-test-suite/tool
dart pub get
cd -
```

Then run the test suite (up to the point I've implemented):
```
make test
```
