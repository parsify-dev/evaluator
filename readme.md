# Evaluator

> Experimental expression evaluator for [@parsify/math](https://github.com/parsify-dev/math), powered by Rust & WASM.

[![Build Status](https://travis-ci.com/parsify-dev/evaluator.svg?branch=master)](https://travis-ci.com/parsify-dev/evaluator)

## Development

### Prerequisites

- [Node.js](https://nodejs.org/en/)
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

``` bash
# Build for Node.js
$ wasm-pack build --target nodejs

# Run tests, formatter and linter
$ cargo fmt --all -- --check && cargo clippy && cargo test
```

## License

MIT