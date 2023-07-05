# Lox interpreter test suite

These tests were all written by Bob Nystrom.

## Testing

This works with Dart 2.12 but not the latest. `brew install dart@2.12`

Run `dart pub get` in ./tool to get deps.

I've copied in Nystrom's Lox interpreter test suite. From his testing docs:

```
If you had an interpreter executable at my_code/boblox, you could test it like:

$ dart tool/bin/test.dart clox --interpreter my_code/boblox

You still need to tell it which suite of tests to run because that determines the test expectations. If your interpreter should behave like jlox, use "jlox" as the suite name. If it behaves like clox, use "clox". If your interpreter is only complete up to the end of one of the chapters in the book, you can use that chapter as the suite, like "chap10_functions".

If your interpreter needs other command line arguments passed to use, pass them to the test runner using --arguments and it will forward to your interpreter.
```

chap04_scanning
chap05_representing
chap06_parsing
chap07_evaluating
chap08_statements
chap09_control
chap10_functions
chap11_resolving
chap12_classes
chap13_inheritance
chap14_chunks
chap15_virtual
chap16_scanning
chap17_compiling
chap18_types
chap19_strings
chap20_hash
chap21_global
chap22_local
chap23_jumping
chap24_calls
chap25_closures
chap26_garbage
chap27_classes
chap28_methods
chap29_superclasses
chap30_optimization
