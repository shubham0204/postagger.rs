# Parts of Speech Tagging - Perceptron Tagger in Rust

## Generate C headers and shared library

```
$> cbindgen --lang C --output postagger.h
$> cargo build --target=x86_64-unknown-linux-gnu --release
```